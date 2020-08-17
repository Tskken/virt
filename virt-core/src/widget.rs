use crate::decoder::{WidgetConfig, DecoderError, Shapes, Tools};
use crate::geometry::*;
use crate::util::Color;
use crate::tools::Button;
use crate::action::Action;

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
    pub fn new(config: WidgetConfig) -> Result<Widget, DecoderError> {
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
                                return Err(DecoderError::InvalidShapeFormat);
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
                                return Err(DecoderError::InvalidShapeFormat);
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
                                return Err(DecoderError::InvalidShapeFormat);
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
                                                Some(Action::new(a, None)),
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
                                   
                                    //widget.buttons.push(rectangle.color(Color::from_hex(hex::decode(&c[1..])?)));
                                },
                                None => {
                                    match t.action {
                                        Some(a) => {
                                           let button = Button::new(
                                               rectangle,
                                               Some(Action::new(a, None)),
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

    pub fn to_vec(&self) -> Vec<Vector> {
        let mut d: Vec<Vector> = Vec::new();

        for shape in &self.shapes {
            d.append(&mut shape.to_vec());
        }

        for button in &self.buttons {
            d.append(&mut button.shape.to_vec());
        }

        d
    }
}