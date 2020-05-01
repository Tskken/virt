use std::ops::{Add, Sub, Mul, Div};

/// Rectangle is your standard 2D rectangular shape. This holds two Point's,
/// one being the minimum point and the other people the maximum point.
/// This does mean that when creating the Rectangle with new(), you should
/// not set the max point to be less then the mid point otherwise most functions
/// provided will ether error, or give undefined behaviors.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rectangle {
    pub min: Point,
    pub max: Point,
}

impl Rectangle {
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Rectangle {
        Rectangle {
            min: Point::new(min_x, min_y),
            max: Point::new(max_x, max_y),
        }
    }

    pub fn center(self) -> Point {
        self.max - Point::new(self.half_width(), self.half_height())
    }

    pub fn width(self) -> f64 {
        self.max.x - self.min.x
    }

    pub fn height(self) -> f64 {
        self.max.y - self.min.y
    }

    pub fn half_width(self) -> f64 {
        self.width() / 2f64
    }

    pub fn half_height(self) -> f64 {
        self.height() / 2f64
    }

    pub fn area(self) -> f64 {
        self.width() * self.height()
    }

    pub fn contains(self, p: Point) -> bool {
        !(p.x < self.min.x || p.x > self.max.x || p.y < self.min.y || p.y > self.max.y)
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

impl Add<Point> for Rectangle {
    type Output = Rectangle;

    fn add(self, v: Point) -> Rectangle {
        Rectangle {
            min: self.min + v,
            max: self.max + v,
        }
    }
}

impl Add<f64> for Rectangle {
    type Output = Rectangle;

    fn add(self, v: f64) -> Rectangle {
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

impl Sub<Point> for Rectangle {
    type Output = Rectangle;

    fn sub(self, v: Point) -> Rectangle {
        Rectangle {
            min: self.min - v,
            max: self.max - v,
        }
    }
}

impl Sub<f64> for Rectangle {
    type Output = Rectangle;

    fn sub(self, v: f64) -> Rectangle {
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

impl Mul<Point> for Rectangle {
    type Output = Rectangle;

    fn mul(self, v: Point) -> Rectangle {
        Rectangle {
            min: self.min * v,
            max: self.max * v,
        }
    }
}

impl Mul<f64> for Rectangle {
    type Output = Rectangle;

    fn mul(self, v: f64) -> Rectangle {
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

impl Div<Point> for Rectangle {
    type Output = Rectangle;

    fn div(self, v: Point) -> Rectangle {
        Rectangle {
            min: self.min / v,
            max: self.max / v,
        }
    }
}

impl Div<f64> for Rectangle {
    type Output = Rectangle;

    fn div(self, v: f64) -> Rectangle {
        Rectangle {
            min: self.min / v,
            max: self.max / v,
        }
    }
}

// #[derive(Debug, PartialEq, Copy)]
// pub struct Triangle<T: Copy> {
//     pub a: Point<T>,
//     pub b: Point<T>,
//     pub c: Point<T>,
// }

// impl<T: Copy> Triangle<T> {
//     pub fn new(a: Point<T>, b: Point<T>, c: Point<T>) -> Triangle<T> {
//         Triangle {
//             a,
//             b,
//             c,
//         }
//     }
// }

// impl<T> Triangle<T> {
//     pub fn center(self) -> Point<T> {
//         (self.a + self.b + self.c) / Point::new(T::from(3), T::from(3))
//     }
// }

// impl<T: Add<Output = T> + Copy> Add for Triangle<T> {
//     type Output = Triangle<T>;

//     fn add(self, v: Triangle<T>) -> Triangle<T> {
//         Triangle {
//             a: self.a + v.a,
//             b: self.b + v.b,
//             c: self.c + v.c,
//         }
//     }
// }

// impl<T: Sub<Output = T> + Copy> Sub for Triangle<T> {
//     type Output = Triangle<T>;

//     fn sub(self, v: Triangle<T>) -> Triangle<T> {
//         Triangle {
//             a: self.a - v.a,
//             b: self.b - v.b,
//             c: self.c - v.c,
//         }
//     }
// }

// impl<T: Mul<Output = T> + Copy> Mul for Triangle<T> {
//     type Output = Triangle<T>;

//     fn mul(self, v: Triangle<T>) -> Triangle<T> {
//         Triangle {
//             a: self.a * v.a,
//             b: self.b * v.b,
//             c: self.c * v.c,
//         }
//     }
// }

// impl<T: Div<Output = T> + Copy> Div for Triangle<T> {
//     type Output = Triangle<T>;

//     fn div(self, v: Triangle<T>) -> Triangle<T> {
//         Triangle {
//             a: self.a / v.a,
//             b: self.b / v.b,
//             c: self.c / v.c,
//         }
//     }
// }

// impl<T: Copy> Clone for Triangle<T> {
//     fn clone(&self) -> Self {
//         *self
//     }
// }

/// Point is a simple 2D point, typically understood as a Vector, that all geometric shapes use.
/// Point stores a generic T type, but any given type has to implement a few traits.
/// All T given to Point must support PartialEq, Copy, and be able to use Add, Sub, Mul, and Div.
/// This means values given to Point should primarily be number values like ints, uints, and floats.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x,
            y,
        }
    }

    pub fn eq(self, v: Point) -> bool {
        self == v
    }

    pub fn floor(self) -> Self {
        Point {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    pub fn hypot(self) -> f64 {
        self.x.hypot(self.y)
    }

    pub fn angle(self) -> f64 {
        self.x.atan2(self.y)
    }

    pub fn unit(self) -> Point {
        if self.x == 0f64 && self.y == 0f64 {
            return Point {
                x: 1f64,
                y: 0f64,
            }
        }

        self * (1f64 / self.hypot())
    }

    pub fn rotated(self, angle: f64) -> Point {
        let (sin, cos) = angle.sin_cos();

        Point {
            x: self.x * cos - self.y * sin, 
            y: self.x * sin + self.y * cos,
        }
    }

    pub fn normal(self) -> Point {
        self / self.hypot()
    }

    pub fn dot(self, v: Point) -> f64 {
        self.x * v.x + self.y * v.y
    }

    pub fn cross(self, v: Point) -> f64 {
        self.x * v.y - v.x * self.y
    }

    pub fn project(self, v: Point) -> Point {   
        Point {
            x: v.x * self.x.abs() + 0.5,
            y: v.y * self.y.abs() - 0.5,
        }
    }

    pub fn unproject(self, v: Point) -> Point {
        Point {
            x: self.x / v.x - 0.5,
            y: self.y / v.y + 0.5,
        }
    }

    pub fn lerp(self, v: Point, th: f64) -> Point {
        self * 1f64 - th + v * th
    }
}

impl Add<Point> for Point {
    type Output = Point;
    
    fn add(self, v: Point) -> Point {
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Add<f64> for Point {
    type Output = Point;
    
    fn add(self, v: f64) -> Point {
        Point {
            x: self.x + v,
            y: self.y + v,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    
    fn sub(self, v: Point) -> Point {
        Point {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl Sub<f64> for Point {
    type Output = Point;
    
    fn sub(self, v: f64) -> Point {
        Point {
            x: self.x - v,
            y: self.y - v,
        }
    }
}

impl Mul<Point> for Point {
    type Output = Point;
    
    fn mul(self, v: Point) -> Point {
        Point {
            x: self.x * v.x,
            y: self.y * v.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    
    fn mul(self, v: f64) -> Point {
        Point {
            x: self.x * v,
            y: self.y * v,
        }
    }
}

impl Div<Point> for Point {
    type Output = Point;
    
    fn div(self, v: Point) -> Point {
        Point {
            x: self.x / v.x,
            y: self.y / v.y,
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, v: f64) -> Point {
        Point {
            x: self.x / v,
            y: self.y / v,
        }
    }
}