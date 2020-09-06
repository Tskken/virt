use vulkano::device::Device;
use vulkano::framebuffer::{RenderPassAbstract, Subpass};
use vulkano::pipeline::{GraphicsPipelineAbstract, GraphicsPipeline};

use std::sync::Arc;

use crate::geometry::Vector;
use crate::error::Result;

pub struct ShapesPipeline {
    pub default_fill: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    pub default_line: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    //pub circle_fill: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
}

pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
        #version 450

        layout(location = 0) in vec2 position;
        layout(location = 1) in vec4 color;
        
        layout(push_constant) uniform PushConstantData {
            vec2 resolution;
        } pc;
        
        layout(location = 0) out vec4 v_color;
        
        void main() {
            v_color = color;
        
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

pub mod vs_circle {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
        #version 460

        layout(location = 0) in vec2 position;
        layout(location = 1) in vec4 color;
        
        layout(push_constant) uniform PushConstantData {
            vec2 bounds;
            vec2 center;
        } pc;
        
        layout(location = 0) out vec4 v_color;
        layout(location = 1) out vec2 v_bounds;
        layout(location = 2) out vec2 v_center;
        
        void main() {
            v_color = color;
        
            vec2 norm = (position + 0.5) / (pc.bounds / 2.0) - vec2(1.0, 1.0);

            v_bounds = pc.bounds;

            v_center = (pc.center + 0.5) / (pc.bounds / 2.0) - vec2(1.0, 1.0);
        
            gl_Position = vec4(norm, 0.0, 1.0);
        }",
    }
}

pub mod fs_circle {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
            #version 460

            layout(location = 0) in vec4 v_color;
            layout(location = 1) in vec2 v_bounds;
            layout(location = 2) in vec2 v_center;

            layout(location = 0) out vec4 f_color;

            float circle(in vec2 _st, in float _radius){
                vec2 dist = _st-v_center-vec2(0.5);
                return 1.-smoothstep(_radius-(_radius*0.01),
                                     _radius+(_radius*0.01),
                                     dot(dist,dist)*4.0);
            }
            
            void main(){
                vec2 st = gl_FragCoord.xy/v_bounds;
                st.x *= v_bounds.x / v_bounds.y;
            
                float color = circle(st,0.9);
            
                f_color = vec4( v_color.rgb * color, color );
            }
        ",}
}

// mod cfs {
//     vulkano_shaders::shader! {
//         ty: "fragment",
//         path: "src/bin/shaders/circle_frag.spv",
//     }
// }

// pub fn new_default(device: Arc<Device>, render_pass: Arc<dyn RenderPassAbstract + Send + Sync>) -> Result<Arc<dyn GraphicsPipelineAbstract + Send + Sync>> {
//     Ok(Arc::new(
//             GraphicsPipeline::start()
//                 .vertex_input_single_buffer::<Vector>()
//                 .vertex_shader(vs::Shader::load(device.clone()).unwrap().main_entry_point(), ())
//                 .viewports_dynamic_scissors_irrelevant(1)
//                 .fragment_shader(fs::Shader::load(device.clone()).unwrap().main_entry_point(), ())
//                 .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
//                 .build(device.clone())?,
//         )
//     )
// }


impl ShapesPipeline {
    pub fn new(device: Arc<Device>, render_pass: Arc<dyn RenderPassAbstract + Send + Sync>) -> Result<ShapesPipeline> {
        Ok(ShapesPipeline {
            default_fill: Arc::new(
                GraphicsPipeline::start()
                    .vertex_input_single_buffer::<Vector>()
                    .vertex_shader(vs::Shader::load(device.clone()).unwrap().main_entry_point(), ())
                    .viewports_dynamic_scissors_irrelevant(1)
                    .fragment_shader(fs::Shader::load(device.clone()).unwrap().main_entry_point(), ())
                    .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                    .build(device.clone())?,
            ),
            default_line: Arc::new(
                GraphicsPipeline::start()
                    .vertex_input_single_buffer::<Vector>()
                    .vertex_shader(vs::Shader::load(device.clone()).unwrap().main_entry_point(), ())
                    .line_strip()
                    .line_width(3.0)
                    .viewports_dynamic_scissors_irrelevant(1)
                    .fragment_shader(fs::Shader::load(device.clone()).unwrap().main_entry_point(), ())
                    .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                    .build(device.clone())?,
            ),
            // default_fill: Arc::new(
            //     GraphicsPipeline::start()
            //         .vertex_input_single_buffer::<Vector>()
            //         .vertex_shader(vs_circle::Shader::load(device.clone()).unwrap().main_entry_point(), ())
            //         //.point_list()
            //         .viewports_dynamic_scissors_irrelevant(1)
            //         .fragment_shader(fs_circle::Shader::load(device.clone()).unwrap().main_entry_point(), ())
            //         .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            //         .build(device.clone())?,
            // ),
        })
    }
}