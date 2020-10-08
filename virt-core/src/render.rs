// use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
// use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
// use vulkano::device::Device;

// use cgmath::Vector2;

// use std::sync::Arc;
// use std::ops::Deref;

// use crate::pipelines::{ShapesPipeline, vs::ty::PushConstantData};
// use crate::shapes::Shape;
// use crate::error::{Result, CoreError};

// pub struct Renderer {
//     builder: AutoCommandBufferBuilder,
//     piplelines: ShapesPipeline,
//     resolution: Vector2<f32>,
// }

// impl Renderer {
//     pub fn draw(&self, device: Arc<Device>, shape: &dyn Shape, dynamic_state: &DynamicState) -> Result<()> {
//         let (vertexes, indexes) = shape.vertexes();

//         let vertex_buffer = CpuAccessibleBuffer::from_iter(
//             device.clone(), BufferUsage::all(), false, 
//             vertexes.iter().cloned()
//         ).unwrap();

//         match indexes {
//             Some(i) => {
//                 let index_buffer = CpuAccessibleBuffer::from_iter(
//                     device.clone(), BufferUsage::all(), false, 
//                     i.iter().clone()
//                 ).unwrap();

//                 self.builder.draw_indexed(
//                     self.piplelines.default_fill, 
//                     dynamic_state, 
//                     vec![vertex_buffer.clone()], 
//                     index_buffer.clone(), 
//                     (),
//                     PushConstantData {
//                         resolution: [self.resolution.x, self.resolution.y]
//                     }
//                 );

//                 return Ok(())
//             },
//             None => {
//                 return Ok(())
//             }
//         }

//         //Err(CoreError::Unimplemented)
//     }
// }