use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Debug;
use std::sync::Arc;

use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::device::Device;

use crate::pipelines::{ShapesPipeline, vs};
use crate::buffer::Buffer;
use crate::vector::Vector;
use crate::color::Color;
use crate::error::{CoreError, Result};

pub trait Shape : Debug + Send + Sync {
    fn center(&self) -> Vector;
    fn area(&self) -> f32;

    fn color(&mut self, c: Color);
    fn format(&mut self, f: ShapeFormat);

    fn contains(&self, v: Vector) -> bool;

    fn draw(&mut self,
        device: Arc<Device>,
        builder: &mut AutoCommandBufferBuilder,
        pipelines: &ShapesPipeline,
        dynamic_state: &DynamicState,
        resolution: Vector,
    );
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShapeFormat {
    Fill,
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub position: Vector,
    pub wh: Vector,

    pub color: Color,

    pub format: ShapeFormat,

    pub buffer: Option<Buffer>,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rectangle {
        Rectangle {
            position: Vector::new(x, y),
            wh: Vector::new(w, h),
            color: Color::default(),
            format: ShapeFormat::Fill,
            buffer: None,
        }
    }
}

impl Shape for Rectangle {
    fn center(&self) -> Vector {
        self.position + (self.wh / 2f32)
    }
    fn area(&self) -> f32 {
        self.wh.x() * self.wh.y()
    }

    fn color(&mut self, c: Color) {
        self.color = c;
    }
    fn format(&mut self, f: ShapeFormat) {
        self.format = f;
    }

    fn contains(&self, v: Vector) -> bool {
        !(v.x() < self.position.x() || v.x() > self.position.x() + self.wh.x() || v.y() < self.position.y() || v.y() > self.position.y() + self.wh.y())
    }

    fn draw(&mut self,
        device: Arc<Device>,
        builder: &mut AutoCommandBufferBuilder,
        pipelines: &ShapesPipeline,
        dynamic_state: &DynamicState,
        resolution: Vector,
    ) {
        match &self.buffer {
            Some(b) => {
                let col = self.color.to_float();
        
                builder.draw_indexed(
                    pipelines.default_fill.clone(), 
                    dynamic_state, 
                    vec![b.vertex_buffer.clone()], 
                    b.index_buffer.clone(),
                    (),
                    vs::ty::PushConstantData {
                        resolution: [resolution.x(), resolution.y()],
                        r: col[0],
                        g: col[1],
                        b: col[2],
                        a: col[3],
                    }
                ).unwrap();
            },
            None => {
                let vertex_buffer = CpuAccessibleBuffer::from_iter(
                    device.clone(), BufferUsage::all(), false, 
                    vec![   self.position,
                        self.position + Vector::new(self.wh.x(), 0f32),
                        self.position + Vector::new(0f32, self.wh.y()),
                        self.position + self.wh
                    ].iter().cloned()
                ).unwrap();
        
                let index_buffer = CpuAccessibleBuffer::from_iter(
                    device.clone(), BufferUsage::all(), false, 
                    vec![0u16, 1u16, 2u16, 1u16, 3u16, 2u16].iter().cloned()
                ).unwrap();
        
                self.buffer = Some(Buffer::new(vertex_buffer, index_buffer));
            }
        }
        // let vertex_buffer = CpuAccessibleBuffer::from_iter(
        //     device.clone(), BufferUsage::all(), false, 
        //     vec![   self.position,
        //         self.position + Vector::new(self.wh.x(), 0f32),
        //         self.position + Vector::new(0f32, self.wh.y()),
        //         self.position + self.wh
        //     ].iter().cloned()
        // ).unwrap();

        // let index_buffer = CpuAccessibleBuffer::from_iter(
        //     device.clone(), BufferUsage::all(), false, 
        //     vec![0u16, 1u16, 2u16, 1u16, 3u16, 2u16].iter().cloned()
        // ).unwrap();

        // let col = self.color.to_float();

        // builder.draw_indexed(
        //     pipelines.default_fill.clone(), 
        //     dynamic_state, 
        //     vec![vertex_buffer.clone()], 
        //     index_buffer.clone(),
        //     (),
        //     vs::ty::PushConstantData {
        //         resolution: [resolution.x(), resolution.y()],
        //         r: col[0],
        //         g: col[1],
        //         b: col[2],
        //         a: col[3],
        //     }
        // ).unwrap();
    }
}

#[macro_export]
macro_rules! rec {
    ($( $x:expr , $y:expr , $w:expr , $h:expr)*) => {
        {
            $(
                Rectangle::new($x, $y, $w, $h)
            )*
        }
    };

    ($( $w:expr , $h:expr)*) => {
        {
            $(
                Rectangle::new(0f32, 0f32, $w, $h)
            )*
        }
    };

    ($( $x:expr , $y:expr , $w:expr , $h:expr),*) => {
        {
            let mut v = Vec::new();
            $(
                v.push(Rectangle::new($x, $y, $w, $h));
            )*
            v
        }
    };
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Vector,
    pub b: Vector,
    pub c: Vector,

    pub color: Color,
    pub format: ShapeFormat,

    pub buffer: Option<Buffer>,
}

impl Triangle {
    pub fn new(a: Vector, b: Vector, c: Vector) -> Triangle {
        Triangle {
            a,
            b,
            c,

            color: Color::default(),
            format: ShapeFormat::Fill,

            buffer: None,
        }
    }
}

impl Shape for Triangle {
    fn center(&self) -> Vector {
        (self.a + self.b + self.c) / 3f32
    }
    fn area(&self) -> f32 {
        ((self.a.x() * (self.b.y() - self.c.y())
        + self.b.x() * (self.c.y() - self.a.y())
        + self.c.x() * (self.a.y() - self.b.y())
        ) / 2f32).abs()
    }

    fn color(&mut self, c: Color) {
        self.color = c;
    }
    fn format(&mut self, f: ShapeFormat) {
        self.format = f;
    }

    fn contains(&self, v: Vector) -> bool {
        self.area() == Triangle::new(v, self.b, self.c).area() + Triangle::new(self.a, v, self.c).area() + Triangle::new(self.a, self.b, v).area()
    }

    fn draw(&mut self,
        device: Arc<Device>,
        builder: &mut AutoCommandBufferBuilder,
        pipelines: &ShapesPipeline,
        dynamic_state: &DynamicState,
        resolution: Vector,
    ) {
        match &self.buffer {
            Some(b) => {
                let col = self.color.to_float();

                builder.draw_indexed(
                    pipelines.default_fill.clone(), 
                    dynamic_state, 
                    vec![b.vertex_buffer.clone()],
                    b.index_buffer.clone(),
                    (),
                    vs::ty::PushConstantData {
                        resolution: [resolution.x(), resolution.y()],
                        r: col[0],
                        g: col[1],
                        b: col[2],
                        a: col[3],
                    },
                ).unwrap();
            },
            None => {
                let vertex_buffer = CpuAccessibleBuffer::from_iter(
                    device.clone(), BufferUsage::all(), false, 
                    vec![self.a, self.b, self.c].iter().cloned()
                ).unwrap();
        
                let index_buffer = CpuAccessibleBuffer::from_iter(
                    device.clone(), BufferUsage::all(), false, 
                    vec![0u16, 1u16, 2u16].iter().cloned()
                ).unwrap();
        
                self.buffer = Some(Buffer::new(vertex_buffer, index_buffer));
            }
        }
    }
}

#[macro_export]
macro_rules! tri {
    ($( $xa:expr , $ya:expr , $xb:expr , $yb:expr, $xc:expr , $yc:expr )*) => {
        {
            $(
                Triangle::new(
                    Vector2::new($xa, $ya),
                    Vector2::new($xb, $yb),
                    Vector2::new($xc, $yc)
                )
            )*
        }
    };

    ($( $xa:expr , $ya:expr , $xb:expr , $yb:expr, $xc:expr , $yc:expr ),*) => {
        {
            let mut v = Vec::new();
            $(
                v.push(Triangle::new(
                    Vector2::new($xa, $ya),
                    Vector2::new($xb, $yb),
                    Vector2::new($xc, $yc)
                ));
            )*
            v
        }
    };
}

