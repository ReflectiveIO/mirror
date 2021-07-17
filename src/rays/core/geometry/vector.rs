use std::f32::consts::PI;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use crate::rays::geometry::Point;
use crate::rays::utils::clamp;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vector {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Vector { x, y, z } }

    pub fn length_squared(&self) -> f32 { self.x * self.x + self.y * self.y + self.z * self.z }

    pub fn length(&self) -> f32 { f32::sqrt(self.length_squared()) }
}

impl From<f32> for Vector {
    fn from(v: f32) -> Self { Vector::new(v, v, v) }
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

/// The division operator /.
impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output { Self::new(self.x / rhs, self.y / rhs, self.z / rhs) }
}

#[inline]
pub fn dot(v1: &Vector, v2: &Vector) -> f32 { v1.x * v2.x + v1.y * v2.y + v1.z * v2.z }

#[inline]
pub fn normalize(v: &Vector) -> Vector { v.clone() / v.length() }

#[inline]
pub fn spherical_theta(v: &Vector) -> f32 { clamp(v.z, -1.0, 1.0).acos() }

#[inline]
pub fn spherical_phi(v: &Vector) -> f32 {
    let p: f32 = v.y.atan2(v.x);

    if p < 0.0 {
        p + 2.0 * PI
    } else {
        p
    }
}
