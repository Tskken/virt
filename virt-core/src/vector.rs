use std::ops::{Add, Sub, Mul, Div};

use crate::color::Color;

/// Vector is a simple 2D point that all geometric shapes use.
#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vector {
    pub position: [f32; 2],
}

impl Vector {
    // Create a new Vector
    pub fn new(x: f32, y: f32) -> Vector {
        Vector {
            position: [x, y],
        }
    }

    pub fn x(&self) -> f32 {
        self.position[0]
    }

    pub fn y(&self) -> f32 {
        self.position[1]
    }
    
    // Floor returns a new Vector with this vectors x and y coordinates rounded down (floored)
    pub fn floor(&self) -> Self {
        Vector {
            position: [
                self.position[0].floor(),
                self.position[1].floor(),
            ],
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
        if self.x() == 0f32 && self.y() == 0f32 {
            return Vector {
                position: [1f32, 0f32],
            }
        }

        self * (1f32 / self.hypot())
    }

    pub fn rotated(&self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        Vector {
            position: [
                self.position[0] * cos - self.position[1] * sin,
                self.position[0] * sin + self.position[1] * cos,
            ]
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

    pub fn unproject(&self, v: Vector) -> Self {
        let len = self.dot(v) / v.hypot();
        v.unit() * len
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
                ]
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
                ]
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
                ]
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
                ]
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
                ]
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
                ]
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
                ]
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
                ]
        }
    }
}