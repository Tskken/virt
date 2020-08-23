use std::ops::{Add, Sub, Mul, Div};
use core::fmt::Debug;

use crate::util::Color;
use crate::error::{CoreError, Result};

use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::buffer::CpuBufferPool;
use vulkano::pipeline::GraphicsPipelineAbstract;

use std::sync::Arc;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShapeFormat {
    Fill,
    Line,
}

pub trait Shape : Debug {
    fn center(&self) -> Vector;
    fn area(&self) -> f32;
    fn color(&self, color: [f32; 4]) -> Box<dyn Shape>;
    fn contains(&self, p: Vector) -> bool;
    fn project(&self, width: f32, height: f32) -> Box<dyn Shape>;
    fn draw(&self,
        builder: &mut AutoCommandBufferBuilder, 
        buffer_pool: &CpuBufferPool<Vector>,
        pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
        dynamic_state: &DynamicState
    ) -> Result<()>;
}

/// Rectangle is your standard 2D rectangular shape.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rectangle {
    pub min: Vector,
    pub max: Vector,

    pub format: ShapeFormat,
}

impl Rectangle {
    pub fn new(min: Vector, max: Vector) -> Rectangle {
        Rectangle {
            min,
            max,
            format: ShapeFormat::Line,
        }
    }

    fn width(self) -> f32 {
        self.max.x() - self.min.x()
    }

    fn height(self) -> f32 {
        self.max.y() - self.min.y()
    }

    fn to_fill_vec(&self) -> Vec<Vector> {
        vec![
            self.min,
            Vector::new(self.min.x(), self.max.y()).color(self.max.color),
            self.max,
            self.min,
            Vector::new(self.max.x(), self.min.y()).color(self.max.color),
            self.max,
        ]
    }

    fn to_line_vec(&self) -> Vec<Vector> {
        vec![
            self.min,
            Vector::new(self.max.x(), self.min.y()).color(self.max.color),
            self.max,
            Vector::new(self.min.x(), self.max.y()).color(self.max.color),
            self.min,
        ]
    }
}

impl Shape for Rectangle {
    fn color(&self, color: [f32; 4]) -> Box<dyn Shape> {
        Box::new(Rectangle {
            min: self.min.color(color),
            max: self.max.color(color),
            format: self.format,
        })
    }

    fn area(&self) -> f32 {
        self.width() * self.height()
    }

    fn center(&self) -> Vector {
        self.max - Vector::new(self.width() / 2f32, self.height() / 2f32)
    }

    fn contains(&self, p: Vector) -> bool {
        !(p.position[0] < self.min.position[0] || p.position[0] > self.max.position[0] || p.position[1] > self.min.position[1] || p.position[1] < self.max.position[1])
    }

    fn project(&self, width: f32, height: f32) -> Box<dyn Shape> {
        Box::new(Rectangle {
            min: self.min.project(width, height),
            max: self.max.project(width, height),
            format: self.format,
        })
    }

    fn draw(&self, 
        builder: &mut AutoCommandBufferBuilder, 
        buffer_pool: &CpuBufferPool<Vector>,
        pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
        dynamic_state: &DynamicState,) -> Result<()>{
            let buffer = Arc::new(buffer_pool.chunk(self.to_fill_vec().clone())?);

            builder.draw(
                pipeline.clone(),
                &dynamic_state,
                vec![buffer],
                (),
                (),
            )?;

        // match self.format {
        //     ShapeFormat::Fill => {
                
        //     },
        //     ShapeFormat::Line => {
        //         let buffer = Arc::new(buffer_pool.chunk(self.to_line_vec().clone())?);

        //         builder.draw(
        //             pipeline.clone(),
        //             &dynamic_state,
        //             vec![buffer],
        //             (),
        //             (),
        //         )?;
        //     }
        // }

        Ok(())
    }
}

impl Add<Rectangle> for Rectangle {
    type Output = Rectangle;

    fn add(self, v: Rectangle) -> Self::Output {
        Rectangle {
            min: self.min + v.min,
            max: self.max + v.max,
            format: self.format,
        }
    }
}

impl Add<Vector> for Rectangle {
    type Output = Rectangle;

    fn add(self, v: Vector) -> Self::Output {
        Rectangle {
            min: self.min + v,
            max: self.max + v,
            format: self.format,
        }
    }
}

impl Add<f32> for Rectangle {
    type Output = Rectangle;

    fn add(self, v: f32) -> Self::Output {
        Rectangle {
            min: self.min + v,
            max: self.max + v,
            format: self.format,
        }
    }
}

impl Sub<Rectangle> for Rectangle {
    type Output = Rectangle;
    
    fn sub(self, v: Rectangle) -> Self::Output {
        Rectangle {
            min: self.min - v.min,
            max: self.max - v.max,
            format: self.format,
        }
    }
}

impl Sub<Vector> for Rectangle {
    type Output = Rectangle;

    fn sub(self, v: Vector) -> Self::Output {
        Rectangle {
            min: self.min - v,
            max: self.max - v,
            format: self.format,
        }
    }
}

impl Sub<f32> for Rectangle {
    type Output = Rectangle;

    fn sub(self, v: f32) -> Self::Output {
        Rectangle {
            min: self.min - v,
            max: self.max - v,
            format: self.format,
        }
    }
}

impl Mul<Rectangle> for Rectangle {
    type Output = Rectangle;

    fn mul(self, v: Rectangle) -> Self::Output {
        Rectangle {
            min: self.min * v.min,
            max: self.max * v.max,
            format: self.format,
        }
    }
}

impl Mul<Vector> for Rectangle {
    type Output = Rectangle;

    fn mul(self, v: Vector) -> Self::Output {
        Rectangle {
            min: self.min * v,
            max: self.max * v,
            format: self.format,
        }
    }
}

impl Mul<f32> for Rectangle {
    type Output = Rectangle;

    fn mul(self, v: f32) -> Self::Output {
        Rectangle {
            min: self.min * v,
            max: self.max * v,
            format: self.format,
        }
    }
}

impl Div<Rectangle> for Rectangle {
    type Output = Rectangle;

    fn div(self, v: Rectangle) -> Self::Output {
        Rectangle {
            min: self.min / v.min,
            max: self.max / v.max,
            format: self.format,
        }
    }
}

impl Div<Vector> for Rectangle {
    type Output = Rectangle;

    fn div(self, v: Vector) -> Self::Output {
        Rectangle {
            min: self.min / v,
            max: self.max / v,
            format: self.format,
        }
    }
}

impl Div<f32> for Rectangle {
    type Output = Rectangle;

    fn div(self, v: f32) -> Self::Output {
        Rectangle {
            min: self.min / v,
            max: self.max / v,
            format: self.format,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub a: Vector,
    pub b: Vector,
    pub c: Vector,

    pub format: ShapeFormat,
}

impl Triangle {
    pub fn new(a: Vector, b: Vector, c: Vector) -> Triangle {
        Triangle {
            a,
            b,
            c,
            format: ShapeFormat::Line,
        }
    }

    fn to_fill_vec(&self) -> Vec<Vector> {
        vec![
            self.a,
            self.b,
            self.c,
        ]
    }

    fn to_line_vec(&self) -> Vec<Vector> {
        vec![
            self.a,
            self.b,
            self.c,
            self.a,
        ]
    }
}

impl Shape for Triangle {
    fn area(&self) -> f32 {
        (
            (self.a.position[0] * (self.b.position[1] - self.c.position[1]) 
            + self.b.position[0] * (self.c.position[1] - self.a.position[1]) 
            + self.c.position[0] * (self.a.position[1] - self.b.position[1])
        )/2f32).abs()
    }

    fn contains(&self, p: Vector) -> bool {
        self.area() == Triangle::new(p, self.b, self.c).area() + Triangle::new(self.a, p, self.c).area() + Triangle::new(self.a, self.b, p).area()
    }

    fn color(&self, color: [f32; 4]) -> Box<dyn Shape> {
        Box::new(Triangle {
            a: self.a.color(color),
            b: self.b.color(color),
            c: self.c.color(color),
            format: self.format,
        })
    }

    fn center(&self) -> Vector {
        (self.a + self.b + self.c) / Vector::new(3f32, 3f32)
    }

    fn project(&self, width: f32, height: f32) -> Box<dyn Shape> {
        Box::new(Triangle {
            a: self.a.project(width, height),
            b: self.b.project(width, height),
            c: self.c.project(width, height),
            format: self.format,
        })
    }

    fn draw(&self,
        builder: &mut AutoCommandBufferBuilder, 
        buffer_pool: &CpuBufferPool<Vector>,
        pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
        dynamic_state: &DynamicState
    ) -> Result<()> {
        let buffer = Arc::new(buffer_pool.chunk(self.to_fill_vec().clone())?);

        builder.draw(
            pipeline.clone(),
            &dynamic_state,
            vec![buffer],
            (),
            (),
        )?;

        // match self.format {
        //     ShapeFormat::Fill => {

        //     },
        //     ShapeFormat::Line => {
        //         let buffer = Arc::new(buffer_pool.chunk(self.to_line_vec().clone())?);

        //         builder.draw(
        //             shapes_pipeline.default_line.clone(),
        //             &dynamic_state,
        //             vec![buffer],
        //             (),
        //             (),
        //         )?;
        //     }
        // }
        

        Ok(())
    }
}

impl Add<Triangle> for Triangle {
    type Output = Triangle;

    fn add(self, v: Triangle) -> Self::Output {
        Triangle {
            a: self.a + v.a,
            b: self.b + v.b,
            c: self.c + v.c,
            format: self.format,
        }
    }
}

impl Add<f32> for Triangle {
    type Output = Triangle;

    fn add(self, v: f32) -> Self::Output {
        Triangle {
            a: self.a + v,
            b: self.b + v,
            c: self.c + v,
            format: self.format,
        }
    }
}

impl Sub<Triangle> for Triangle {
    type Output = Triangle;

    fn sub(self, v: Triangle) -> Self::Output {
        Triangle {
            a: self.a - v.a,
            b: self.b - v.b,
            c: self.c - v.c,
            format: self.format,
        }
    }
}

impl Sub<f32> for Triangle {
    type Output = Triangle;

    fn sub(self, v: f32) -> Self::Output {
        Triangle {
            a: self.a - v,
            b: self.b - v,
            c: self.c - v,
            format: self.format,
        }
    }
}

impl Mul<Triangle> for Triangle {
    type Output = Triangle;

    fn mul(self, v: Triangle) -> Self::Output {
        Triangle {
            a: self.a * v.a,
            b: self.b * v.b,
            c: self.c * v.c,
            format: self.format,
        }
    }
}

impl Mul<f32> for Triangle {
    type Output = Triangle;

    fn mul(self, v: f32) -> Self::Output {
        Triangle {
            a: self.a * v,
            b: self.b * v,
            c: self.c * v,
            format: self.format,
        }
    }
}

impl Div<Triangle> for Triangle {
    type Output = Triangle;

    fn div(self, v: Triangle) -> Self::Output {
        Triangle {
            a: self.a / v.a,
            b: self.b / v.b,
            c: self.c / v.c,
            format: self.format,
        }
    }
}

impl Div<f32> for Triangle {
    type Output = Triangle;

    fn div(self, v: f32) -> Self::Output {
        Triangle {
            a: self.a / v,
            b: self.b / v,
            c: self.c / v,
            format: self.format,
        }
    }
}

impl From<[Vector; 3]> for Triangle {
    fn from(data: [Vector; 3]) -> Triangle {
        Triangle {
            a: data[0],
            b: data[1],
            c: data[2],
            format: ShapeFormat::Fill,
        }
    }
}

#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Circle {
    pub center: Vector,
    pub radius: Vector,
}

impl Circle {
    pub fn new(center: Vector, radius: Vector) -> Circle {
        Circle {
            center,
            radius,
        }
    }
}

impl Shape for Circle {
    fn center(&self) -> Vector {
        self.center
    }

    fn area(&self) -> f32 {
        0f32
    }

    fn color(&self, color: [f32; 4]) -> Box<dyn Shape> {
        Box::new(Circle {
            center: self.center.color(color),
            radius: self.radius,
        })
    }

    fn contains(&self, _p: Vector) -> bool {
        false
    }

    fn project(&self, width: f32, height: f32) -> Box<dyn Shape> {
        Box::new(Circle {
            center: self.center.project(width, height),
            radius: self.radius.project(width, height),
        })
    }

    fn draw(&self,
        _builder: &mut AutoCommandBufferBuilder, 
        _buffer_pool: &CpuBufferPool<Vector>,
        _pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
        _dynamic_state: &DynamicState
    ) -> Result<()> {
        Err(CoreError::Unimplemented)
    }
}

impl Add<Circle> for Circle {
    type Output = Circle;

    fn add(self, c: Circle) -> Self {
        Circle {
            center: self.center,
            radius: self.radius + c.radius,
        }
    }
}

impl Add<Vector> for Circle {
    type Output = Circle;

    fn add(self, v: Vector) -> Self {
        Circle {
            center: self.center + v,
            radius: self.radius,
        }
    }
}

impl Add<f32> for Circle {
    type Output = Circle;

    fn add(self, v: f32) -> Self {
        Circle {
            center: self.center + v,
            radius: self.radius,
        }
    }
}

impl Sub<Circle> for Circle {
    type Output = Circle;

    fn sub(self, c: Circle) -> Self {
        Circle {
            center: self.center,
            radius: self.radius - c.radius,
        }
    }
}

impl Sub<Vector> for Circle {
    type Output = Circle;

    fn sub(self, v: Vector) -> Self {
        Circle {
            center: self.center - v,
            radius: self.radius,
        }
    }
}

impl Sub<f32> for Circle {
    type Output = Circle;

    fn sub(self, v: f32) -> Self {
        Circle {
            center: self.center - v,
            radius: self.radius,
        }
    }
}

impl Mul<Vector> for Circle {
    type Output = Circle;

    fn mul(self, v: Vector) -> Self {
        Circle {
            center: self.center,
            radius: self.radius * v,
        }
    }
}

impl Mul<f32> for Circle {
    type Output = Circle;

    fn mul(self, v: f32) -> Self {
        Circle {
            center: self.center,
            radius: self.radius * v,
        }
    }
}

impl Div<Vector> for Circle {
    type Output = Circle;

    fn div(self, v: Vector) -> Self {
        Circle {
            center: self.center,
            radius: self.radius / v,
        }
    }
}

impl Div<f32> for Circle {
    type Output = Circle;

    fn div(self, v: f32) -> Self {
        Circle {
            center: self.center,
            radius: self.radius / v,
        }
    }
}

/// Vector is a simple 2D point that all geometric shapes use.
#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vector {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl Vector {
    // Create a new Vector
    pub fn new(x: f32, y: f32) -> Vector {
        Vector {
            position: [x, y],
            color: Color::none(),
        }
    }

    pub fn x(&self) -> f32 {
        self.position[0]
    }

    pub fn y(&self) -> f32 {
        self.position[1]
    }

    pub fn color(self, color: [f32; 4]) -> Self {
        Vector {
            position: self.position,
            color,
        }
    }
    
    // Floor returns a new Vector with this vectors x and y coordinates rounded down (floored)
    pub fn floor(self) -> Self {
        Vector {
            position: [
                self.position[0].floor(),
                self.position[1].floor(),
            ],
            color: self.color,
        }
    }

    // Hypot gets the length of this vector using the hypot method of a f32.
    pub fn hypot(&self) -> f32 {
        self.position[0].hypot(self.position[1])
    }

    // Returns the atan2 angle of this vector
    pub fn angle(&self) -> f32 {
        self.position[0].atan2(self.position[1])
    }

    pub fn unit(self) -> Self {
        if self.position[0] == 0f32 && self.position[1] == 0f32 {
            return Vector {
                position: [
                    1f32,
                    0f32,
                ],
                color: self.color,
            }
        }

        self * (1f32 / self.hypot())
    }

    pub fn rotated(self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        Vector {
            position: [
                self.position[0] * cos - self.position[1] * sin,
                self.position[0] * sin + self.position[1] * cos,
            ],
            color: self.color,
        }
    }

    pub fn normal(self) -> Self {
        self / self.hypot()
    }

    pub fn dot(&self, v: Vector) -> f32 {
        self.position[0] * v.position[0] + self.position[1] * v.position[1]
    }

    pub fn cross(&self, v: Vector) -> f32 {
        self.position[0] * v.position[1] - v.position[0] * self.position[1]
    }

    pub fn project(self, width: f32, height: f32) -> Self {
        Vector {
            position: [
                (self.position[0] + 0.5f32) / (width / 2f32) - 1f32,
                1f32 - (self.position[1] + 0.5f32) / (height / 2f32),
            ],
            color: self.color,
        }
    }

    pub fn lerp(self, v: Vector, th: f32) -> Self {
        self * 1f32 - th + v * th
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    
    fn add(self, v: Vector) -> Self::Output {
        Vector {
            position: [
                self.position[0] + v.position[0], 
                self.position[1] + v.position[1]
                ],
            color: self.color,
        }
    }
}

impl Add<f32> for Vector {
    type Output = Vector;
    
    fn add(self, v: f32) -> Self::Output {
        Vector {
            position: [
                self.position[0] + v, 
                self.position[1] + v,
                ],
            color: self.color,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    
    fn sub(self, v: Vector) -> Self::Output {
        Vector {
            position: [
                self.position[0] - v.position[0], 
                self.position[1] - v.position[1]
                ],
                color: self.color,
        }
    }
}

impl Sub<f32> for Vector {
    type Output = Vector;
    
    fn sub(self, v: f32) -> Self::Output {
        Vector {
            position: [
                self.position[0] - v, 
                self.position[1] - v,
                ],
                color: self.color,
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;
    
    fn mul(self, v: Vector) -> Self::Output {
        Vector {
            position: [
                self.position[0] * v.position[0], 
                self.position[1] * v.position[1]
                ],
                color: self.color,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;
    
    fn mul(self, v: f32) -> Self::Output {
        Vector {
            position: [
                self.position[0] * v, 
                self.position[1] * v,
                ],
                color: self.color,
        }
    }
}

impl Div<Vector> for Vector {
    type Output = Vector;
    
    fn div(self, v: Vector) -> Self::Output {
        Vector {
            position: [
                self.position[0] / v.position[0], 
                self.position[1] / v.position[1]
                ],
                color: self.color,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;
    
    fn div(self, v: f32) -> Self::Output {
        Vector {
            position: [
                self.position[0] / v, 
                self.position[1] / v,
                ],
                color: self.color,
        }
    }
}

impl From<[f32; 2]> for Vector {
    fn from(data: [f32; 2]) -> Vector {
        Vector {
            position: data,
            color: Color::none(),
        }
    }
}

impl From<[f32; 6]> for Vector {
    fn from(data: [f32; 6]) -> Vector {
        Vector {
            position: [
                data[0],
                data[1],
            ],
            color: [
                data[2],
                data[3],
                data[4],
                data[5],
            ],
        }
    }
}