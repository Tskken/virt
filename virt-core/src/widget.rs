use crate::decoder::{WidgetConfig, Shapes, Tools};
use crate::error::{CoreError, Result};
use crate::geometry::*;
use crate::util::Color;
use crate::tools::Button;
use crate::action::Action;

use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::buffer::CpuBufferPool;
use vulkano::framebuffer::FramebufferAbstract;
use vulkano::pipeline::GraphicsPipelineAbstract;

use std::sync::Arc;

#[derive(Debug)]
pub struct Widget {
    pub width: u32,
    pub height: u32,

    pub position: Vector,

    pub color: [f32; 4],

    pub shapes: Vec<Box<dyn Shape>>,

    pub buttons: Vec<Button>,
}

impl Widget {
    pub fn new(config: WidgetConfig) -> Result<Widget> {
        let mut widget = Widget{
            width: config.width,
            height: config.height,
            position: Vector::new(config.position[0], config.position[1]),
            color: Color::none(),
            shapes: Vec::new(),
            buttons: Vec::new(),
        };

        match config.color {
            Some(c) => {
                widget.color = Color::from_hex(hex::decode(&c[1..])?);
            },
            None => (),
        };

        match config.shape {
            Some(shapes) => {
                for s in shapes {
                    match s.shape_type {
                        Shapes::Triangle => {
                            if s.shape.len() != 6 {
                                return Err(CoreError::InvalidShapeFormat);
                            };

                            let triangle = Triangle::new(
                                Vector::new(s.shape[0], s.shape[1]),
                                Vector::new(s.shape[2], s.shape[3]),
                                Vector::new(s.shape[4], s.shape[5]),
                            )
                            .project(config.width as f32, config.height as f32);

                            match s.color {
                                Some(c) => {
                                    widget.shapes.push(triangle.color(Color::from_hex(hex::decode(&c[1..])?)));
                                },
                                None => {
                                    widget.shapes.push(triangle);
                                }
                            }
                        },
                        Shapes::Rectangle => {
                            if s.shape.len() != 4 {
                                return Err(CoreError::InvalidShapeFormat);
                            };

                            let rectangle = Rectangle::new(
                                Vector::new(s.shape[0], s.shape[1]), 
                                Vector::new(s.shape[2], s.shape[3])
                            )
                            .project(config.width as f32, config.height as f32);

                            match s.color {
                                Some(c) => {
                                    widget.shapes.push(rectangle.color(Color::from_hex(hex::decode(&c[1..])?)));
                                },
                                None => {
                                    widget.shapes.push(rectangle);
                                }
                            };
                        },
                    };
                };
            },
            None => (),
        };

        match config.tool {
            Some(tools) => {
                for t in tools {
                    match t.tool_type {
                        Tools::Button => {
                            if t.shape.len() != 4 {
                                return Err(CoreError::InvalidShapeFormat);
                            };

                            let rectangle = Rectangle::new(
                                Vector::new(t.shape[0], t.shape[1]), 
                                Vector::new(t.shape[2], t.shape[3])
                            )
                            .project(config.width as f32, config.height as f32);

                            match t.color {
                                Some(c) => {
                                    match t.action {
                                         Some(a) => {
                                            let button = Button::new(
                                                rectangle.color(Color::from_hex(hex::decode(&c[1..])?)),
                                                Some(Action::new(a)),
                                            );

                                            widget.buttons.push(button);
                                         },
                                         None => {
                                            let button = Button::new(
                                                rectangle.color(Color::from_hex(hex::decode(&c[1..])?)),
                                                None,
                                            );

                                            widget.buttons.push(button);
                                         }
                                     }
                                },
                                None => {
                                    match t.action {
                                        Some(a) => {
                                           let button = Button::new(
                                               rectangle,
                                               Some(Action::new(a)),
                                           );

                                           widget.buttons.push(button);
                                        },
                                        None => {
                                           let button = Button::new(
                                               rectangle,
                                               None,
                                           );

                                           widget.buttons.push(button);
                                        }
                                    }
                                }
                            };
                        },
                    };
                };
            },
            None => (),
        };


        Ok(widget)
    }

    pub fn draw(
        &self,
        builder: &mut AutoCommandBufferBuilder, 
        buffer_pool: &CpuBufferPool<Vector>,
        frame_buffer: Arc<dyn FramebufferAbstract + Send + Sync>,
        pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
        dynamic_state: &DynamicState,
    ) -> Result<()> {

        builder
            .begin_render_pass(frame_buffer.clone(), false, vec![self.color.into()])?;

        for shape in &self.shapes {
            let buffer = Arc::new(buffer_pool.chunk(shape.to_vec().clone())?);

            builder.draw(
                pipeline.clone(),
                &dynamic_state,
                vec![buffer],
                (),
                (),
            )?;
        }

        for button in &self.buttons {
            let buffer = Arc::new(buffer_pool.chunk(button.shape.to_vec().clone())?);

            builder.draw(
                pipeline.clone(),
                &dynamic_state,
                vec![buffer],
                (),
                (),
            )?;
        }

        Ok(())
    }
}