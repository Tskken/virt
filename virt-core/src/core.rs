use vulkano::buffer::{CpuBufferPool};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::framebuffer::{FramebufferAbstract, RenderPassAbstract, Subpass};
use vulkano::image::{ImageUsage};
use vulkano::instance::{Instance, PhysicalDevice, PhysicalDeviceType};
// use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
// use vulkano::pipeline::vertex::SingleBufferDefinition;
// use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::swapchain;
use vulkano::swapchain::{
    AcquireError, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
    SwapchainCreationError, Surface, SwapchainAcquireFuture
};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};

use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit::dpi::{LogicalSize, LogicalPosition};

use std::sync::Arc;
use std::collections::HashMap;

use crate::util::*;
use crate::geometry::Vector;
use crate::decoder;
use crate::widget::Widget;

pub struct CoreSurface {
    surface: Arc<Surface<Window>>,
    swapchain: Arc<Swapchain<Window>>,
    //images: Vec<Arc<SwapchainImage<Window>>>,
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: DynamicState,
    recreate_swapchain: bool,
    previous_frame_end: Option<Box<(dyn GpuFuture)>>,
    framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    
}

impl CoreSurface {
    fn new(physical: &PhysicalDevice, device: Arc<Device>, queue: Arc<Queue>, event_loop: &EventLoop<()>, instance: Arc<Instance>, widget: &Widget) -> Result<CoreSurface, &'static str> {
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
                //images,
                render_pass,
                dynamic_state,
                recreate_swapchain,
                previous_frame_end,
                framebuffers,
            }
        )
    }
}

pub fn run() {
    let instance = Instance::new(None, &vulkano_win::required_extensions(), None).unwrap();
    let physical = Arc::new(find_device(&instance, PhysicalDeviceType::IntegratedGpu).unwrap());

    let event_loop = EventLoop::new();

    let queue_family = physical
    .queue_families()
    .find(|&q| {
        q.supports_graphics()
    })
    .unwrap();

    let device_ext = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };
    let (device, mut queues) = Device::new(
        *physical,
        physical.supported_features(),
        &device_ext,
        [(queue_family, 0.5)].iter().cloned(),
    )
    .unwrap();

    let queue = queues.next().unwrap();

    let widget_config = decoder::decode("C:/Users/dillb/Documents/Rust_Projects/virt/virt-core/src/bin/widget_files/all_widgets.toml").unwrap();

    let widget = Widget::new(widget_config).unwrap();

    let surface = CoreSurface::new(&physical, device.clone(), queue.clone(), &event_loop, instance.clone(), &widget).unwrap();

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

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(_),
                ..
            } => {
                surfaces
                    .get_mut(&window_id)
                    .unwrap()
                    .recreate_swapchain = true;
            }
            Event::RedrawEventsCleared => {
                surfaces
                    .values()
                    .for_each(|s| s.surface.window().request_redraw());
            }
            Event::RedrawRequested(window_id) => {
                let mut surface = surfaces.get_mut(&window_id).unwrap();

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
                    // Because framebuffers contains an Arc on the old swapchain, we need to
                    // recreate framebuffers as well.
                    surface.framebuffers = window_size_dependent_setup(
                        &new_images,
                        surface.render_pass.clone(),
                        &mut surface.dynamic_state,
                    );
                    surface.recreate_swapchain = false;
                }

                // Before we can draw on the output, we have to *acquire* an image from the swapchain. If
                // no image is available (which happens if you submit draw commands too quickly), then the
                // function will block.
                // This operation returns the index of the image that we are allowed to draw upon.
                //
                // This function can block if no image is available. The parameter is an optional timeout
                // after which the function call will return an error.
                let (image_num, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(surface.swapchain.clone(), None) {
                        Ok(r) => r,
                        Err(AcquireError::OutOfDate) => {
                            surface.recreate_swapchain = true;
                            return;
                        }
                        Err(e) => panic!("Failed to acquire next image: {:?}", e),
                    };

                // acquire_next_image can be successful, but suboptimal. This means that the swapchain image
                // will still work, but it may not display correctly. With some drivers this can be when
                // the window resizes, but it may not cause the swapchain to become out of date.
                if suboptimal {
                    surface.recreate_swapchain = true;
                }

                draw(surface, image_num, acquire_future, buffer_pool.clone(), device.clone(), queue.clone(), pipeline.clone(), &widget).unwrap();
            }
            _ => (),
        }
    });
}

fn draw(
    surface: &mut CoreSurface, 
    image_num: usize, 
    acquire_future: SwapchainAcquireFuture<Window>,
    buffer_pool: CpuBufferPool<Vector>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync + 'static>,
    widget: &Widget,
) -> Result<(), &'static str>{
    // // Specify the color to clear the framebuffer with i.e. blue
    // let clear_values = vec![[0.0, 0.0, 0.0, 0.0].into()];

    // let data =  Triangle::new(Vector::new(-0.5, -0.25), Vector::new(0.0, 0.5), Vector::new(0.25, -0.1))
    // .color(1f32, 0f32, 0f32, 1f32)
    // .to_vec();

    // let data = match widget.triangles.get(0) {
    //     Some(t) => t.to_vec(),
    //     None => panic!("Err in getting triangles from widget!"),
    // };

    let data = {
        let mut d = Vec::new();

        for triangle in &widget.triangles {
            d.push(triangle.a);
            d.push(triangle.b);
            d.push(triangle.c);
        };

        d
    };

    let clear_values = vec![widget.color.into()];

    let buffer = Arc::new(buffer_pool.chunk(data.clone()).unwrap());

    // In order to draw, we have to build a *command buffer*. The command buffer object holds
    // the list of commands that are going to be executed.
    //
    // Building a command buffer is an expensive operation (usually a few hundred
    // microseconds), but it is known to be a hot path in the driver and is expected to be
    // optimized.
    //
    // Note that we have to pass a queue family when we create the command buffer. The command
    // buffer will only be executable on that given queue family.
    let mut builder = AutoCommandBufferBuilder::primary_one_time_submit(
        device.clone(),
        queue.family(),
    )
    .unwrap();

    builder
        // Before we can draw, we have to *enter a render pass*. There are two methods to do
        // this: `draw_inline` and `draw_secondary`. The latter is a bit more advanced and is
        // not covered here.
        //
        // The third parameter builds the list of values to clear the attachments with. The API
        // is similar to the list of attachments when building the framebuffers, except that
        // only the attachments that use `load: Clear` appear in the list.
        .begin_render_pass(surface.framebuffers[image_num].clone(), false, clear_values)
        .unwrap()
        // We are now inside the first subpass of the render pass. We add a draw command.
        //
        // The last two parameters contain the list of resources to pass to the shaders.
        // Since we used an `EmptyPipeline` object, the objects have to be `()`.
        .draw(
            pipeline.clone(),
            &surface.dynamic_state,
            vec![buffer],
            (),
            (),
        )
        .unwrap()
        // We leave the render pass by calling `draw_end`. Note that if we had multiple
        // subpasses we could have called `next_inline` (or `next_secondary`) to jump to the
        // next subpass.
        .end_render_pass()
        .unwrap();

    // Finish building the command buffer by calling `build`.
    let command_buffer = builder.build().unwrap();

    let future = surface.previous_frame_end
        .take()
        .unwrap()
        .join(acquire_future)
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        // The color output is now expected to contain our triangle. But in order to show it on
        // the screen, we have to *present* the image by calling `present`.
        //
        // This function does not actually present the image immediately. Instead it submits a
        // present command at the end of the queue. This means that it will only be presented once
        // the GPU has finished executing the command buffer that draws the triangle.
        .then_swapchain_present(queue.clone(), surface.swapchain.clone(), image_num)
        .then_signal_fence_and_flush();

    match future {
        Ok(future) => {
            surface.previous_frame_end = Some(future.boxed());
        }
        Err(FlushError::OutOfDate) => {
            surface.recreate_swapchain = true;
            surface.previous_frame_end = Some(sync::now(device.clone()).boxed());
        }
        Err(_) => {
            surface.previous_frame_end = Some(sync::now(device.clone()).boxed());

            return Err("Failed to flush future");
        }
    }

    Ok(())
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