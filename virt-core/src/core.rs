use vulkano::buffer::{CpuBufferPool};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::framebuffer::{FramebufferAbstract, RenderPassAbstract, Subpass};
use vulkano::image::{ImageUsage};
use vulkano::instance::{Instance, PhysicalDevice, PhysicalDeviceType};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::swapchain;
use vulkano::swapchain::{
    AcquireError, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
    SwapchainCreationError, Surface
};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};

use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder, WindowId};
use winit::dpi::{LogicalSize, LogicalPosition};

use std::sync::Arc;
use std::collections::HashMap;

use crate::util::*;
use crate::geometry::Vector;
use crate::decoder;
use crate::widget::Widget;
use crate::decoder::WidgetConfig;

pub struct CoreState {
    pub instance: Arc<Instance>,
    pub physical_index: usize,

    pub queue: Arc<Queue>,
    pub device: Arc<Device>,

    pub buffer_pool: CpuBufferPool<Vector>,

    pub pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,

    pub surfaces: HashMap<WindowId, CoreSurface>,
}

impl CoreState {
    pub fn new() -> (CoreState, EventLoop<()>){
        let instance = Instance::new(None, &vulkano_win::required_extensions(), None).unwrap();
        let physical_index = find_device_index(instance.clone(), PhysicalDeviceType::IntegratedGpu).unwrap();

        let event_loop = EventLoop::new();

        let physical = PhysicalDevice::from_index(&instance, physical_index).unwrap();

        let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .unwrap();

        let device_ext = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };
        let (device, mut queues) = Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned(),
        )
        .unwrap();

        let queue = queues.next().unwrap();

        let widget_config = decoder::decode("C:/Users/dillb/Documents/Rust_Projects/virt/virt-core/src/bin/widget_files/all_widgets.toml").unwrap();

        let surface = CoreSurface::new(&physical, device.clone(), queue.clone(), &event_loop, instance.clone(), widget_config).unwrap();

        let widget_config2 = decoder::decode("C:/Users/dillb/Documents/Rust_Projects/virt/virt-core/src/bin/widget_files/all_widgets2.toml").unwrap();

        let surface2 = CoreSurface::new(&physical, device.clone(), queue.clone(), &event_loop, instance.clone(), widget_config2).unwrap();

        vulkano::impl_vertex!(Vector, position, color);

        let buffer_pool: CpuBufferPool<Vector> = CpuBufferPool::vertex_buffer(device.clone());

        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();

        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<Vector>()
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .render_pass(Subpass::from(surface.render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        let mut surfaces = HashMap::new();

        surfaces.insert(surface.surface.window().id(), surface);
        surfaces.insert(surface2.surface.window().id(), surface2);

        (CoreState {
            instance,
            physical_index,
            queue,
            device,
            buffer_pool,
            pipeline,
            surfaces,
        },
        event_loop)
    }

    pub fn draw(&mut self, surface_id: WindowId) {
        let mut surface = self.surfaces.get_mut(&surface_id).unwrap();

        surface.previous_frame_end.as_mut().unwrap().cleanup_finished();

        if surface.recreate_swapchain {
            let dimensions: [u32; 2] = surface.surface.window().inner_size().into();
            let (new_swapchain, new_images) =
                match surface.swapchain.recreate_with_dimensions(dimensions) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::UnsupportedDimensions) => return,
                    Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                };

            surface.swapchain = new_swapchain;
            surface.framebuffers = window_size_dependent_setup(
                &new_images,
                surface.render_pass.clone(),
                &mut surface.dynamic_state,
            );
            surface.recreate_swapchain = false;
        }

        let (image_num, suboptimal, acquire_future) =
            match swapchain::acquire_next_image(surface.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    surface.recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };

        if suboptimal {
            surface.recreate_swapchain = true;
        }


        let clear_values = vec![surface.widget.color.into()];
    
        let buffer = Arc::new(self.buffer_pool.chunk(surface.widget.to_vec().clone()).unwrap());
    
        let mut builder = AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.queue.family(),
        )
        .unwrap();
    
        builder
            .begin_render_pass(surface.framebuffers[image_num].clone(), false, clear_values)
            .unwrap()
            .draw(
                self.pipeline.clone(),
                &surface.dynamic_state,
                vec![buffer],
                (),
                (),
            )
            .unwrap()
            .end_render_pass()
            .unwrap();
    
        let command_buffer = builder.build().unwrap();
    
        let future = surface.previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(self.queue.clone(), surface.swapchain.clone(), image_num)
            .then_signal_fence_and_flush();
    
        match future {
            Ok(future) => {
                surface.previous_frame_end = Some(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                surface.recreate_swapchain = true;
                surface.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(_) => {
                surface.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
    
                panic!("Failed to flush future");
            }
        }
    }
}

pub struct CoreSurface {
    pub surface: Arc<Surface<Window>>,
    pub swapchain: Arc<Swapchain<Window>>,
    pub render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    pub dynamic_state: DynamicState,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<(dyn GpuFuture)>>,
    pub framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    
    pub widget: Widget,

    pub cur_mouse_pos: Option<Vector>,
    pub las_mouse_pos: Option<Vector>,
}

impl CoreSurface {
    fn new(physical: &PhysicalDevice, device: Arc<Device>, queue: Arc<Queue>, event_loop: &EventLoop<()>, instance: Arc<Instance>, config: WidgetConfig) -> Result<CoreSurface, &'static str> {
        let widget = Widget::new(config).unwrap();

        let surface = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(widget.width, widget.height))
        .with_decorations(false)    
        .with_transparent(true)
        .with_resizable(false)
        .build_vk_surface(&event_loop, instance)
        .unwrap();

        surface.window().set_outer_position(LogicalPosition::new(widget.position.x(), widget.position.y()));

        let (swapchain, images) = {
            let caps = surface.capabilities(*physical).unwrap();

            let alpha = caps.supported_composite_alpha.iter().next().unwrap();

            let format = caps.supported_formats[0].0;

            let dimensions: [u32; 2] = surface.window().inner_size().into();

            Swapchain::new(
                device.clone(),
                surface.clone(),
                caps.min_image_count,
                format,
                dimensions,
                1,
                ImageUsage::color_attachment(),
                &queue,
                SurfaceTransform::Identity,
                alpha,
                PresentMode::Fifo,
                FullscreenExclusive::Default,
                true,
                ColorSpace::SrgbNonLinear,
            )
            .unwrap()
        };

        let render_pass = Arc::new(
            vulkano::single_pass_renderpass!(
                device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: swapchain.format(),
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            )
            .unwrap(),
        );

        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
            compare_mask: None,
            write_mask: None,
            reference: None,
        };

        let framebuffers =
            window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);

        let recreate_swapchain = false;


        let previous_frame_end = Some(sync::now(device.clone()).boxed());

        Ok(
            CoreSurface {
                surface,
                swapchain,
                render_pass,
                dynamic_state,
                recreate_swapchain,
                previous_frame_end,
                framebuffers,
                widget: widget,
                cur_mouse_pos: None,
                las_mouse_pos: None,
            }
        )
    }
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/bin/shaders/vert.vs"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/bin/shaders/frag.fs"
    }
}