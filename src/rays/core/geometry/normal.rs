use std::ops::*;

use super::{Cross, Dot, Vector};
use crate::rays::geometry::Matrix4x4;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Normal {
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

    pub fn add_with_weight(&mut self, src: &Normal, weight: f32) {
        self.x += weight * src.x;
        self.y += weight * src.y;
        self.z += weight * src.z;
    }

    #[inline]
    pub fn normalize(&self) -> Self { self.clone() / self.length() }
}

impl From<Vector> for Normal {
    fn from(v: Vector) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

/// The addition operator +.
impl Add for Normal {
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
impl AddAssign for Normal {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

/// The subtraction operator -.
impl Sub for Normal {
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
impl SubAssign for Normal {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Mul<f32> for Normal {
    type Output = Normal;

    fn mul(self, rhs: f32) -> Self::Output { Self::new(self.x * rhs, self.y * rhs, self.z * rhs) }
}

impl MulAssign<f32> for Normal {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// The division operator /.
impl Div<f32> for Normal {
    type Output = Normal;

    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Self::new(self.x * inv, self.y * inv, self.z * inv)
    }
}

impl DivAssign<f32> for Normal {
    fn div_assign(&mut self, rhs: f32) {
        let inv = 1.0 / rhs;
        *self = Self {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}

impl Neg for Normal {
    type Output = Normal;

    fn neg(self) -> Self::Output { Self::new(-self.x, -self.y, -self.z) }
}

impl Index<usize> for Normal {
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

impl Mul<Normal> for f32 {
    type Output = Normal;

    fn mul(self, rhs: Normal) -> Self::Output { rhs * self }
}

/// Cross(Normal, Vector) -> Vector
impl Cross<Vector> for Normal {
    type Output = Vector;

    #[inline]
    fn cross(&self, rhs: &Vector) -> Self::Output {
        Vector::new(
            (self.y * rhs.z) - (self.z * rhs.y),
            (self.z * rhs.x) - (self.x * rhs.z),
            (self.x * rhs.y) - (self.y * rhs.x),
        )
    }
}

/// Dot(Normal, Normal)
impl Dot for Normal {
    fn dot(&self, rhs: &Self) -> f32 { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z }
}

/// Dot(Normal, Vector)
impl Dot<Vector> for Normal {
    fn dot(&self, rhs: &Vector) -> f32 { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z }
}

/// Normal *= &Matrix4x4
impl MulAssign<&Matrix4x4> for Normal {
    fn mul_assign(&mut self, rhs: &Matrix4x4) {
        let (x, y, z) = (self.x, self.y, self.z);

        self.x = rhs.m[0][0] * x + rhs.m[0][1] * y + rhs.m[0][2] * z;
        self.y = rhs.m[1][0] * x + rhs.m[1][1] * y + rhs.m[1][2] * z;
        self.z = rhs.m[2][0] * x + rhs.m[2][1] * y + rhs.m[2][2] * z;
    }
}
