use vulkano::device::Device;
use vulkano::framebuffer::{RenderPassAbstract, Subpass};
use vulkano::pipeline::{GraphicsPipelineAbstract, GraphicsPipeline};

use std::sync::Arc;

use crate::geometry::Vector;
use crate::error::Result;

// pub struct ShapesPipeline {
//     pub default_fill: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
//     pub default_line: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
// }

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/bin/shaders/vert.spv",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/bin/shaders/frag.spv",
    }
}

// mod cfs {
//     vulkano_shaders::shader! {
//         ty: "fragment",
//         path: "src/bin/shaders/circle_frag.spv",
//     }
// }

pub fn new_default(device: Arc<Device>, render_pass: Arc<dyn RenderPassAbstract + Send + Sync>) -> Result<Arc<dyn GraphicsPipelineAbstract + Send + Sync>> {
    Ok(Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<Vector>()
                .vertex_shader(vs::Shader::load(device.clone()).unwrap().main_entry_point(), ())
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs::Shader::load(device.clone()).unwrap().main_entry_point(), ())
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(device.clone())?,
        )
    )
}


// impl ShapesPipeline {

//             default_line: Arc::new(
//                 GraphicsPipeline::start()
//                     .vertex_input_single_buffer::<Vector>()
//                     .vertex_shader(vs::Shader::load(device.clone()).unwrap().main_entry_point(), ())
//                     .line_strip()
//                     .line_width(3.0)
//                     .viewports_dynamic_scissors_irrelevant(1)
//                     .fragment_shader(fs::Shader::load(device.clone()).unwrap().main_entry_point(), ())
//                     .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
//                     .build(device.clone())?,
//             ),
//         })
//     }
// }