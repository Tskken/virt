use vulkano::device::Device;
use vulkano::framebuffer::{RenderPassAbstract, Subpass};
use vulkano::pipeline::{GraphicsPipelineAbstract, GraphicsPipeline};

use std::sync::Arc;

use crate::vector::Vector;
use crate::error::Result;

pub struct ShapesPipeline {
    pub default_fill: Arc<dyn GraphicsPipelineAbstract + Send + Sync>
}

pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
        #version 450

        layout(location = 0) in vec2 position;
        
        layout(push_constant) uniform PushConstantData {
            vec2 resolution;
            float r;
            float g;
            float b;
            float a;
        } pc;
        
        layout(location = 0) out vec4 v_color;
        
        void main() {
            v_color = vec4(pc.r, pc.g, pc.b, pc.a);
        
            vec2 norm = (position + 0.5) / (pc.resolution / 2.0) - vec2(1.0, 1.0);
        
            gl_Position = vec4(norm, 0.0, 1.0);
        }",
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
        #version 450

        layout(location = 0) in vec4 v_color;

        layout(location = 0) out vec4 f_color;

        void main(){
            f_color = v_color;
        }
    ",}
}

impl ShapesPipeline {
    pub fn new(device: Arc<Device>, render_pass: Arc<dyn RenderPassAbstract + Send + Sync>) -> Result<ShapesPipeline> {
        Ok(ShapesPipeline {
            default_fill: Arc::new(
                GraphicsPipeline::start()
                    .vertex_input_single_buffer::<Vector>()
                    .vertex_shader(vs::Shader::load(device.clone()).unwrap().main_entry_point(), ())
                    .viewports_dynamic_scissors_irrelevant(1)
                    .triangle_list()
                    .fragment_shader(fs::Shader::load(device.clone()).unwrap().main_entry_point(), ())
                    .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                    .build(device.clone())?,
            )
        })
    }
}