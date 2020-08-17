use std::ops::{Add, Sub, Mul, Div};
use crate::util::Color;
use core::fmt::Debug;

pub trait Shape : Debug {
    fn center(&self) -> Vector;
    fn area(&self) -> f32;
    fn color(&self, color: [f32; 4]) -> Box<dyn Shape>;
    fn contains(&self, p: Vector) -> bool;
    fn project(&self, width: f32, height: f32) -> Box<dyn Shape>;
    fn to_vec(&self) -> Vec<Vector>;
}

/// Rectangle is your standard 2D rectangular shape.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rectangle {
    pub min: Vector,
    pub max: Vector,
}

impl Rectangle {
    pub fn new(min: Vector, max: Vector) -> Rectangle {
        Rectangle {
            min,
            max,
        }
    }

    fn width(self) -> f32 {
        self.max.x() - self.min.x()
    }

    fn height(self) -> f32 {
        self.max.y() - self.min.y()
    }

    pub fn area(self) -> f32 {
        self.width() * self.height()
    }
}

impl Shape for Rectangle {
    fn color(&self, color: [f32; 4]) -> Box<dyn Shape> {
        Box::new(Rectangle {
            min: self.min.color(color),
            max: self.max.color(color),
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
        })
    }

    fn to_vec(&self) -> Vec<Vector> {
        vec![
            Vector::new(self.min.x(), self.min.y()).color(self.max.color),
            Vector::new(self.min.x(), self.max.y()).color(self.max.color),
            Vector::new(self.max.x(), self.max.y()).color(self.max.color),
            Vector::new(self.min.x(), self.min.y()).color(self.max.color),
            Vector::new(self.max.x(), self.min.y()).color(self.max.color),
            Vector::new(self.max.x(), self.max.y()).color(self.max.color),
        ]
    }
}

impl Add<Rectangle> for Rectangle {
    type Output = Rectangle;

    fn add(self, v: Rectangle) -> Self::Output {
        Rectangle {
            min: self.min + v.min,
            max: self.max + v.max,
        }
    }
}

impl Add<Vector> for Rectangle {
    type Output = Rectangle;

    fn add(self, v: Vector) -> Self::Output {
        Rectangle {
            min: self.min + v,
            max: self.max + v,
        }
    }
}

impl Add<f32> for Rectangle {
    type Output = Rectangle;

    fn add(self, v: f32) -> Self::Output {
        Rectangle {
            min: self.min + v,
            max: self.max + v,
        }
    }
}

impl Sub<Rectangle> for Rectangle {
    type Output = Rectangle;
    
    fn sub(self, v: Rectangle) -> Self::Output {
        Rectangle {
            min: self.min - v.min,
            max: self.max - v.max,
        }
    }
}

impl Sub<Vector> for Rectangle {
    type Output = Rectangle;

    fn sub(self, v: Vector) -> Self::Output {
        Rectangle {
            min: self.min - v,
            max: self.max - v,
        }
    }
}

impl Sub<f32> for Rectangle {
    type Output = Rectangle;

    fn sub(self, v: f32) -> Self::Output {
        Rectangle {
            min: self.min - v,
            max: self.max - v,
        }
    }
}

impl Mul<Rectangle> for Rectangle {
    type Output = Rectangle;

    fn mul(self, v: Rectangle) -> Self::Output {
        Rectangle {
            min: self.min * v.min,
            max: self.max * v.max,
        }
    }
}

impl Mul<Vector> for Rectangle {
    type Output = Rectangle;

    fn mul(self, v: Vector) -> Self::Output {
        Rectangle {
            min: self.min * v,
            max: self.max * v,
        }
    }
}

impl Mul<f32> for Rectangle {
    type Output = Rectangle;

    fn mul(self, v: f32) -> Self::Output {
        Rectangle {
            min: self.min * v,
            max: self.max * v,
        }
    }
}

impl Div<Rectangle> for Rectangle {
    type Output = Rectangle;

    fn div(self, v: Rectangle) -> Self::Output {
        Rectangle {
            min: self.min / v.min,
            max: self.max / v.max,
        }
    }
}

impl Div<Vector> for Rectangle {
    type Output = Rectangle;

    fn div(self, v: Vector) -> Self::Output {
        Rectangle {
            min: self.min / v,
            max: self.max / v,
        }
    }
}

impl Div<f32> for Rectangle {
    type Output = Rectangle;

    fn div(self, v: f32) -> Self::Output {
        Rectangle {
            min: self.min / v,
            max: self.max / v,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub a: Vector,
    pub b: Vector,
    pub c: Vector,
}

impl Triangle {
    pub fn new(a: Vector, b: Vector, c: Vector) -> Triangle {
        Triangle {
            a,
            b,
            c,
        }
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
        })
    }

    fn to_vec(&self) -> Vec<Vector> {
        vec![
            self.a,
            self.b,
            self.c,
        ]
    }
}

impl Add<Triangle> for Triangle {
    type Output = Triangle;

    fn add(self, v: Triangle) -> Self::Output {
        Triangle {
            a: self.a + v.a,
            b: self.b + v.b,
            c: self.c + v.c,
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
        }
    }
}

impl From<[Vector; 3]> for Triangle {
    fn from(data: [Vector; 3]) -> Triangle {
        Triangle {
            a: data[0],
            b: data[1],
            c: data[2],
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

