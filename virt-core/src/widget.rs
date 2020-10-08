use crate::decoder::{WidgetConfig, Shapes, Tools, Type};
use crate::error::{CoreError, Result};
use crate::shape::*;
use crate::vector::Vector;
use crate::color::Color;
use crate::tools::Button;
use crate::action::{Action, ActionType};
use crate::pipelines::ShapesPipeline;

use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::buffer::CpuBufferPool;
use vulkano::framebuffer::FramebufferAbstract;
use vulkano::device::Device;
use vulkano::format::ClearValue;

use std::sync::Arc;

#[derive(Debug)]
pub struct Widget {
    pub bound: Vector,

    pub position: Vector,

    pub color: Color,

    pub shapes: Vec<Box<dyn Shape>>,

    pub buttons: Vec<Button>,
}

impl Widget {
    pub fn new(config: WidgetConfig) -> Result<Widget> {
        let mut widget = Widget{
            bound: Vector::new(config.width, config.height),
            position: Vector::new(config.position[0], config.position[1]),
            color: Color::default(),
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

                            let mut triangle = Triangle::new(
                                Vector::new(s.shape[0], s.shape[1]),
                                Vector::new(s.shape[2], s.shape[3]),
                                Vector::new(s.shape[4], s.shape[5]),
                            );

                            match s.color {
                                Some(c) => {
                                    triangle.color(Color::from_hex(hex::decode(&c[1..])?));
                                    widget.shapes.push(Box::new(triangle));
                                },
                                None => {
                                    widget.shapes.push(Box::new(triangle));
                                }
                            }
                        },
                        Shapes::Rectangle => {
                            if s.shape.len() != 4 {
                                return Err(CoreError::InvalidShapeFormat);
                            };

                            let mut rectangle = Rectangle::new(
                                s.shape[0], s.shape[1], s.shape[2], s.shape[3]
                            );

                            match s.color {
                                Some(c) => {
                                    rectangle.color(Color::from_hex(hex::decode(&c[1..])?));
                                    widget.shapes.push(Box::new(rectangle));
                                },
                                None => {
                                    widget.shapes.push(Box::new(rectangle));
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
                    match t.ty {
                        Tools::Button => {
                            if t.shape.len() != 4 {
                                return Err(CoreError::InvalidShapeFormat);
                            };

                            let mut rectangle = Rectangle::new(
                                t.shape[0], t.shape[1], t.shape[2], t.shape[3]
                            );

                            match t.color {
                                Some(c) => {
                                    match t.action {
                                         Some(a) => {
                                             match a.ty {
                                                 Type::Clicked => {
                                                    rectangle.color(Color::from_hex(hex::decode(&c[1..])?));
                                                    let button = Button::new(
                                                        Box::new(rectangle),
                                                        Some(Action::new(a.action, ActionType::Clicked)),
                                                    );
        
                                                    widget.buttons.push(button);
                                                 }
                                             }
                                         },
                                         None => {
                                            rectangle.color(Color::from_hex(hex::decode(&c[1..])?));
                                            let button = Button::new(
                                                Box::new(rectangle),
                                                None,
                                            );

                                            widget.buttons.push(button);
                                         }
                                     }
                                },
                                None => {
                                    match t.action {
                                        Some(a) => {
                                            match a.ty {
                                                Type::Clicked => {
                                                    let button = Button::new(
                                                        Box::new(rectangle),
                                                        Some(Action::new(a.action, ActionType::Clicked)),
                                                    );
         
                                                    widget.buttons.push(button);
                                                }
                                            }
                                        },
                                        None => {
                                           let button = Button::new(
                                               Box::new(rectangle),
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
        &mut self,
        device: Arc<Device>,
        builder: &mut AutoCommandBufferBuilder, 
        frame_buffer: Arc<dyn FramebufferAbstract + Send + Sync>,
        pipelines: &ShapesPipeline,
        dynamic_state: &DynamicState,
    ) -> Result<()> {
        builder.begin_render_pass(frame_buffer.clone(), false, vec![ClearValue::Float(self.color.to_float())])?;

        for shape in self.shapes.iter_mut() {
            shape.draw(device.clone(), builder, &pipelines, dynamic_state, self.bound);
        }

        for button in self.buttons.iter_mut() {
            button.shape.draw(device.clone(), builder, &pipelines, dynamic_state, self.bound);
        }

        Ok(())
    }
}