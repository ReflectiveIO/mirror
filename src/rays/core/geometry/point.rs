use std::ops::*;

use config::Value;

use crate::rays::core::geometry::Transform;
use crate::rays::geometry::{Matrix4x4, Vector};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }

    pub fn length_squared(&self) -> f32 { self.x * self.x + self.y * self.y + self.z * self.z }

    pub fn length(&self) -> f32 { f32::sqrt(self.length_squared()) }

    pub fn is_nan(&self) -> bool { self.x.is_nan() || self.y.is_nan() || self.z.is_nan() }

    pub fn is_infinite(&self) -> bool {
        self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite()
    }

    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    pub fn add_with_weight(&mut self, src: &Point, weight: f32) {
        self.x += weight * src.x;
        self.y += weight * src.y;
        self.z += weight * src.z;
    }

    #[inline]
    pub fn distance(&self, p: &Point) -> f32 { (*self - *p).length() }

    #[inline]
    pub fn distance_squared(&self, p: &Point) -> f32 { (*self - *p).length_squared() }
}

impl From<[f32; 3]> for Point {
    fn from(v: [f32; 3]) -> Self {
        Point {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

/// The addition operator +.
/// Point + Point = Point
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// The addition assignment operator +=.
/// Point += Point
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

/// The addition operator +.
/// Point + Vector = Point
impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// The addition assignment operator +=.
/// Point += &Vector
impl AddAssign<&Vector> for Point {
    fn add_assign(&mut self, rhs: &Vector) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

/// The subtraction operator -.
/// Point - Point = Point
impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// &Point - &Point = Vector
impl Sub<&Point> for &Point {
    type Output = Vector;

    fn sub(self, rhs: &Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/// The subtraction assignment operator -=.
/// Point -= Point
impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

/// The subtraction operator -.
/// Point - &Vector = Point
impl Sub<&Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// The subtraction assignment operator -=.
/// Point -= &Vector
impl SubAssign<&Vector> for Point {
    fn sub_assign(&mut self, rhs: &Vector) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output { Self::new(self.x * rhs, self.y * rhs, self.z * rhs) }
}

impl MulAssign<f32> for Point {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// The division operator /.
impl Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Self::new(self.x * inv, self.y * inv, self.z * inv)
    }
}

impl DivAssign<f32> for Point {
    fn div_assign(&mut self, rhs: f32) {
        let inv = 1.0 / rhs;
        *self = Self {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}

impl Index<usize> for Point {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => {
                panic!("Bad index range of Normal")
            },
        }
    }
}

impl Mul<Point> for f32 {
    type Output = Point;

    #[inline]
    fn mul(self, rhs: Point) -> Self::Output { rhs * self }
}

/// Point * &Matrix4x4 = Point
impl Mul<Matrix4x4> for Point {
    type Output = Point;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let (x, y, z) = (self.x, self.y, self.z);

        let mut point = Self {
            x: rhs.m[0][0] * x + rhs.m[0][1] * y + rhs.m[0][2] * z + rhs.m[0][3],
            y: rhs.m[1][0] * x + rhs.m[1][1] * y + rhs.m[1][2] * z + rhs.m[1][3],
            z: rhs.m[2][0] * x + rhs.m[2][1] * y + rhs.m[2][2] * z + rhs.m[2][3],
        };

        let w = rhs.m[3][0] * x + rhs.m[3][1] * y + rhs.m[3][2] * z + rhs.m[3][3];
        if w != 1.0 {
            point /= w;
        }

        return point;
    }
}

/// Point *= &Matrix4x4
impl MulAssign<&Matrix4x4> for Point {
    fn mul_assign(&mut self, rhs: &Matrix4x4) {
        let (x, y, z) = (self.x, self.y, self.z);

        self.x = rhs.m[0][0] * x + rhs.m[0][1] * y + rhs.m[0][2] * z + rhs.m[0][3];
        self.y = rhs.m[1][0] * x + rhs.m[1][1] * y + rhs.m[1][2] * z + rhs.m[1][3];
        self.z = rhs.m[2][0] * x + rhs.m[2][1] * y + rhs.m[2][2] * z + rhs.m[2][3];

        let w = rhs.m[3][0] * x + rhs.m[3][1] * y + rhs.m[3][2] * z + rhs.m[3][3];
        if w != 1.0 {
            *self /= w;
        }
    }
}

/// Point *= &Transform
impl MulAssign<&Transform> for Point {
    fn mul_assign(&mut self, rhs: &Transform) { todo!() }
}

impl Into<Value> for Point {
    fn into(self) -> Value { todo!() }
}
