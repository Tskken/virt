use std::ops::{Add, Sub, Mul, Div};

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

    pub fn color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.min = self.min.color(r, g, b, a);
        self.max = self.max.color(r, g, b, a);

        self
    }

    pub fn center(self) -> Vector {
        self.max - Vector::new(self.width() / 2f32, self.height() / 2f32)
    }

    pub fn width(self) -> f32 {
        self.max.x() - self.min.x()
    }

    pub fn height(self) -> f32 {
        self.max.y() - self.min.y()
    }

    pub fn area(self) -> f32 {
        self.width() * self.height()
    }

    pub fn contains(self, p: Vector) -> bool {
        !(p.x() < self.min.x() || p.x() > self.max.x() || p.y() < self.min.y() || p.y() > self.max.y())
    }

    pub fn project(mut self, width: f32, height: f32) -> Self {
        self.min = self.min.project(width, height);
        self.max = self.max.project(width, height);

        self
    }
}

impl Into<[Triangle; 2]> for Rectangle {
    fn into(self) -> [Triangle; 2] {
        [Triangle::new(
            Vector::new(self.min.x(), self.min.y()),
            Vector::new(self.min.x(), self.max.y()),
            Vector::new(self.max.x(), self.max.y()),
        )
        .color(self.max.color[0], self.max.color[1], self.max.color[2], self.max.color[3]),
        Triangle::new(
            Vector::new(self.min.x(), self.min.y()),
            Vector::new(self.max.x(), self.min.y()),
            Vector::new(self.max.x(), self.max.y()),
        )
        .color(self.max.color[0], self.max.color[1], self.max.color[2], self.max.color[3])
        ]
    }
}

impl Add<Rectangle> for Rectangle {
    type Output = Rectangle;

    fn add(self, v: Rectangle) -> Rectangle {
        Rectangle {
            min: self.min + v.min,
            max: self.max + v.max,
        }
    }
}

impl Add<Vector> for Rectangle {
    type Output = Rectangle;

    fn add(self, v: Vector) -> Rectangle {
        Rectangle {
            min: self.min + v,
            max: self.max + v,
        }
    }
}

impl Add<f32> for Rectangle {
    type Output = Rectangle;

    fn add(self, v: f32) -> Rectangle {
        Rectangle {
            min: self.min + v,
            max: self.max + v,
        }
    }
}

impl Sub<Rectangle> for Rectangle {
    type Output = Rectangle;
    
    fn sub(self, v: Rectangle) -> Rectangle {
        Rectangle {
            min: self.min - v.min,
            max: self.max - v.max,
        }
    }
}

impl Sub<Vector> for Rectangle {
    type Output = Rectangle;

    fn sub(self, v: Vector) -> Rectangle {
        Rectangle {
            min: self.min - v,
            max: self.max - v,
        }
    }
}

impl Sub<f32> for Rectangle {
    type Output = Rectangle;

    fn sub(self, v: f32) -> Rectangle {
        Rectangle {
            min: self.min - v,
            max: self.max - v,
        }
    }
}

impl Mul<Rectangle> for Rectangle {
    type Output = Rectangle;

    fn mul(self, v: Rectangle) -> Rectangle {
        Rectangle {
            min: self.min * v.min,
            max: self.max * v.max,
        }
    }
}

impl Mul<Vector> for Rectangle {
    type Output = Rectangle;

    fn mul(self, v: Vector) -> Rectangle {
        Rectangle {
            min: self.min * v,
            max: self.max * v,
        }
    }
}

impl Mul<f32> for Rectangle {
    type Output = Rectangle;

    fn mul(self, v: f32) -> Rectangle {
        Rectangle {
            min: self.min * v,
            max: self.max * v,
        }
    }
}

impl Div<Rectangle> for Rectangle {
    type Output = Rectangle;

    fn div(self, v: Rectangle) -> Rectangle {
        Rectangle {
            min: self.min / v.min,
            max: self.max / v.max,
        }
    }
}

impl Div<Vector> for Rectangle {
    type Output = Rectangle;

    fn div(self, v: Vector) -> Rectangle {
        Rectangle {
            min: self.min / v,
            max: self.max / v,
        }
    }
}

impl Div<f32> for Rectangle {
    type Output = Rectangle;

    fn div(self, v: f32) -> Rectangle {
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

    pub fn color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.a = self.a.color(r, g, b, a);
        self.b = self.b.color(r, g, b, a);
        self.c = self.c.color(r, g, b, a);

        self
    }

    pub fn center(self) -> Vector {
        (self.a + self.b + self.c) / Vector::new(3f32, 3f32)
    }

    pub fn project(self, width: f32, height: f32) -> Triangle {
        Triangle::new(
            self.a.project(width, height),
            self.b.project(width, height),
            self.c.project(width, height),
        )
    }
}

impl Add<Triangle> for Triangle {
    type Output = Triangle;

    fn add(self, v: Triangle) -> Triangle {
        Triangle {
            a: self.a + v.a,
            b: self.b + v.b,
            c: self.c + v.c,
        }
    }
}

impl Add<f32> for Triangle {
    type Output = Triangle;

    fn add(self, v: f32) -> Triangle {
        Triangle {
            a: self.a + v,
            b: self.b + v,
            c: self.c + v,
        }
    }
}

impl Sub<Triangle> for Triangle {
    type Output = Triangle;

    fn sub(self, v: Triangle) -> Triangle {
        Triangle {
            a: self.a - v.a,
            b: self.b - v.b,
            c: self.c - v.c,
        }
    }
}

impl Sub<f32> for Triangle {
    type Output = Triangle;

    fn sub(self, v: f32) -> Triangle {
        Triangle {
            a: self.a - v,
            b: self.b - v,
            c: self.c - v,
        }
    }
}

impl Mul<Triangle> for Triangle {
    type Output = Triangle;

    fn mul(self, v: Triangle) -> Triangle {
        Triangle {
            a: self.a * v.a,
            b: self.b * v.b,
            c: self.c * v.c,
        }
    }
}

impl Mul<f32> for Triangle {
    type Output = Triangle;

    fn mul(self, v: f32) -> Triangle {
        Triangle {
            a: self.a * v,
            b: self.b * v,
            c: self.c * v,
        }
    }
}

impl Div<Triangle> for Triangle {
    type Output = Triangle;

    fn div(self, v: Triangle) -> Triangle {
        Triangle {
            a: self.a / v.a,
            b: self.b / v.b,
            c: self.c / v.c,
        }
    }
}

impl Div<f32> for Triangle {
    type Output = Triangle;

    fn div(self, v: f32) -> Triangle {
        Triangle {
            a: self.a / v,
            b: self.b / v,
            c: self.c / v,
        }
    }
}

impl From<[Vector; 3]> for Triangle {
    fn from(data: [Vector; 3]) -> Triangle {
        Triangle::new(
            data[0],
            data[1],
            data[2],
        )
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
            color: [0f32; 4],
        }
    }

    pub fn x(self) -> f32 {
        self.position[0]
    }

    pub fn y(self) -> f32 {
        self.position[1]
    }

    pub fn update(mut self, x: f32, y: f32) -> Vector {
        self.position[0] = x;
        self.position[1] = y;

        self
    }

    pub fn update_x(mut self, x: f32) -> Vector {
        self.position[0] = x;

        self
    }

    pub fn update_y(mut self, y: f32) -> Vector {
        self.position[1] = y;

        self
    }

    pub fn color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.color = [r * a, g * a, b * a, a];

        self
    }
    

    // Floor returns a new Vector with this vectors x and y coordinates rounded down (floored)
    pub fn floor(self) -> Self {
        Vector::new(
            self.position[0].floor(), 
            self.position[1].floor()
        )
    }

    // Hypot gets the length of this vector using the hypot method of a f32.
    pub fn hypot(self) -> f32 {
        self.position[0].hypot(self.position[1])
    }

    // Returns the atan2 angle of this vector
    pub fn angle(self) -> f32 {
        self.position[0].atan2(self.position[1])
    }

    pub fn unit(self) -> Vector {
        if self.position[0] == 0f32 && self.position[1] == 0f32 {
            return Vector::new(
                1f32,
                0f32,
            )
        }

        self * (1f32 / self.hypot())
    }

    pub fn rotated(self, angle: f32) -> Vector {
        let (sin, cos) = angle.sin_cos();

        Vector::new(
            self.position[0] * cos - self.position[1] * sin, 
            self.position[0] * sin + self.position[1] * cos,
        )
    }

    pub fn normal(self) -> Vector {
        self / self.hypot()
    }

    pub fn dot(self, v: Vector) -> f32 {
        self.position[0] * v.position[0] + self.position[1] * v.position[1]
    }

    pub fn cross(self, v: Vector) -> f32 {
        self.position[0] * v.position[1] - v.position[0] * self.position[1]
    }

    pub fn project(self, width: f32, height: f32) -> Vector {
        Vector::new(
            (self.position[0] + 0.5f32) / (width / 2f32) - 1f32,
            1f32 - (self.position[1] + 0.5f32) / (height / 2f32),
        )
        /*
        float xClip = (xPix + 0.5f) / 320.0f - 1.0f;
        float yClip = 1.0f - (yPix + 0.5f) / 240.0f;
        */
    }

    // // Project converts world position to screen position
    // pub fn project(self, v: Vector) -> Vector {   
    //     Vector::new(
    //         (self.position[0] + 1.0) / 2.0 * v.position[0],
    //         (1.0 - self.position[1]) / 2.0 * v.position[1],
    //     )
    // }

    // // Unproject converts screen position to world position
    // pub fn unproject(self, v: Vector) -> Vector {
    //     Vector::new(
    //         2.0 * self.position[0] / v.position[0] - 1.0,
    //         2.0 * self.position[1] / v.position[1] - 1.0,
    //     )
    // }

    pub fn lerp(self, v: Vector, th: f32) -> Vector {
        self * 1f32 - th + v * th
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    
    fn add(self, v: Vector) -> Vector {
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
    
    fn add(self, v: f32) -> Vector {
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
    
    fn sub(self, v: Vector) -> Vector {
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
    
    fn sub(self, v: f32) -> Vector {
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
    
    fn mul(self, v: Vector) -> Vector {
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
    
    fn mul(self, v: f32) -> Vector {
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
    
    fn div(self, v: Vector) -> Vector {
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
    
    fn div(self, v: f32) -> Vector {
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
            color: [0f32; 4],
        }
    }
}