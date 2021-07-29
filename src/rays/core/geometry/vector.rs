use std::f32::consts::PI;
use std::ops::*;

use super::{Cross, Dot, Normal, Point};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }

    pub fn length_squared(&self) -> f32 { self.x * self.x + self.y * self.y + self.z * self.z }

    pub fn length(&self) -> f32 { f32::sqrt(self.length_squared()) }

    pub fn is_nan(&self) -> bool { self.x.is_nan() || self.y.is_nan() || self.z.is_nan() }

    pub fn is_infinite(&self) -> bool {
        self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite()
    }

    #[inline]
    pub fn normalize(&self) -> Self { self.clone() / self.length() }
}

impl From<Point> for Vector {
    fn from(v: Point) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Normal> for Vector {
    fn from(v: Normal) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

/// The addition operator +.
impl Add for Vector {
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
impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

/// The subtraction operator -.
impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// The subtraction assignment operator -=.
impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output { Self::new(self.x * rhs, self.y * rhs, self.z * rhs) }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// The division operator /.
impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Self::new(self.x * inv, self.y * inv, self.z * inv)
    }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, rhs: f32) {
        let inv = 1.0 / rhs;
        *self = Self {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output { Self::new(-self.x, -self.y, -self.z) }
}

impl Index<usize> for Vector {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => {
                panic!("Bad index range of Vector")
            },
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output { rhs * self }
}

/// Dot(Vector, Vector)
impl Dot for Vector {
    #[inline]
    fn dot(&self, rhs: &Self) -> f32 { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z }
}

/// Dot(Vector, Normal)
impl Dot<Normal> for Vector {
    fn dot(&self, rhs: &Normal) -> f32 { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z }
}

/// Cross(Vector, Vector) -> Vector
impl Cross for Vector {
    type Output = Vector;

    #[inline]
    fn cross(&self, rhs: &Self) -> Self::Output {
        Self::new(
            (self.y * rhs.z) - (self.z * rhs.y),
            (self.z * rhs.x) - (self.x * rhs.z),
            (self.x * rhs.y) - (self.y * rhs.x),
        )
    }
}

/// Cross(Vector, Normal) -> Vector
impl Cross<Normal> for Vector {
    type Output = Vector;

    #[inline]
    fn cross(&self, rhs: &Normal) -> Self::Output {
        Vector::new(
            (self.y * rhs.z) - (self.z * rhs.y),
            (self.z * rhs.x) - (self.x * rhs.z),
            (self.x * rhs.y) - (self.y * rhs.x),
        )
    }
}
