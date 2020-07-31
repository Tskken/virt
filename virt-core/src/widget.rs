use crate::decoder::{WidgetConfig, DecoderError, Shapes};
use crate::geometry::*;

#[derive(Debug)]
pub struct Widget {
    pub name: Option<String>,

    pub width: u32,
    pub height: u32,

    pub position: Vector,

    pub color: [f32; 4],

    pub triangles: Vec<Triangle>,
}

impl Widget {
    pub fn new(config: WidgetConfig) -> Result<Widget, DecoderError> {
        let mut widget = Widget{
            name: config.name,
            width: config.width,
            height: config.height,
            position: Vector::new(config.position[0], config.position[1]),
            color: [0f32; 4],
            triangles: Vec::new(),
        };

        match config.color {
            Some(c) => {
                let color = hex::decode(&c[1..])?;

                let a = color[3] as f32 / u8::MAX as f32;

                widget.color = [
                    (color[0] as f32 / u8::MAX as f32) * a, 
                    (color[1] as f32 / u8::MAX as f32) * a, 
                    (color[2] as f32 / u8::MAX as f32) * a, 
                    a,
                ];
            },
            None => (),
        };

        match config.shape {
            Some(shapes) => {
                for s in shapes {
                    match s.shape_type {
                        Shapes::Triangle => {
                            if s.shape.len() != 6 {
                                return Err(DecoderError::ValidationFail);
                            };

                            let mut triangle = Triangle::new(
                                Vector::new(s.shape[0], s.shape[1]),
                                Vector::new(s.shape[2], s.shape[3]),
                                Vector::new(s.shape[4], s.shape[5]),
                            )
                            .project(config.width as f32, config.height as f32);

                            match s.color {
                                Some(c) => {
                                    let color = hex::decode(&c[1..])?;
                                    triangle = triangle.color(
                                        color[0] as f32 / u8::MAX as f32, 
                                        color[1] as f32 / u8::MAX as f32, 
                                        color[2] as f32 / u8::MAX as f32, 
                                        color[3] as f32 / u8::MAX as f32,
                                    );
                                },
                                None => (),
                            }

                            widget.triangles.push(triangle);
                        },
                        Shapes::Rectangle => {
                            if s.shape.len() != 4 {
                                return Err(DecoderError::ValidationFail);
                            };

                            let mut rectangle = Rectangle::new(Vector::new(s.shape[0], s.shape[1]), Vector::new(s.shape[2], s.shape[3]))
                            .project(config.width as f32, config.height as f32);

                            // let mut triangles: [Triangle; 2] = Rectangle::new(Vector::new(s.point[0].x, s.point[0].y), Vector::new(s.point[1].x, s.point[1].y))
                            // .project(config.width as f32, config.height as f32)
                            // .into();

                            match s.color {
                                Some(c) => {
                                    let color = hex::decode(&c[1..])?;

                                    rectangle = rectangle.color(
                                        color[0] as f32 / u8::MAX as f32, 
                                        color[1] as f32 / u8::MAX as f32, 
                                        color[2] as f32 / u8::MAX as f32, 
                                        color[3] as f32 / u8::MAX as f32,
                                    );
                                },
                                None => (),
                            }

                            let triangles: [Triangle; 2] = rectangle.into();

                            widget.triangles.push(triangles[0]);
                            widget.triangles.push(triangles[1]);
                        },
                    };
                };
            },
            None => (),
        };

        Ok(widget)
    }
}