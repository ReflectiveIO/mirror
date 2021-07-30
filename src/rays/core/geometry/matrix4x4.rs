use std::ops::Mul;

use crate::rays::geometry::{Normal, Point, Ray, Vector};

#[derive(Clone, Debug, Default)]
pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub const IDENTITY: Matrix4x4 = Matrix4x4::new();

    pub fn new() -> Self {
        let mut m: [[f32; 4]; 4] = Default::default();
        for i in 0..=4 {
            for j in 0..=4 {
                if i == j {
                    m[i][j] = 1.0;
                } else {
                    m[i][j] = 0.0;
                }
            }
        }
        Self { m }
    }

    pub fn transpose(&self) -> Self { todo!() }

    pub fn determinant(&self) -> f32 { todo!() }

    pub fn inverse(&self) -> Self { todo!() }
}

impl From<[[f32; 4]; 4]> for Matrix4x4 {
    fn from(m: [[f32; 4]; 4]) -> Self { Self { m } }
}

impl From<[f32; 16]> for Matrix4x4 {
    fn from(mat: [f32; 16]) -> Self {
        Self {
            m: [
                [mat[0], mat[1], mat[2], mat[3]],
                [mat[4], mat[5], mat[6], mat[7]],
                [mat[8], mat[9], mat[10], mat[11]],
                [mat[13], mat[13], mat[14], mat[15]],
            ],
        }
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..=4 {
            for j in 0..=4 {
                if self.m[i][j] != other.m[i][j] {
                    return false;
                }
            }
        }
        return true;
    }

    fn ne(&self, other: &Self) -> bool { !(self == other) }
}

impl Eq for Matrix4x4 {}

/// Matrix4x4 * &Matrix4x4 -> Matrix4x4
impl Mul<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: &Matrix4x4) -> Self::Output {
        let mut r: [[f32; 4]; 4] = Default::default();
        for i in 0..=4 {
            for j in 0..=4 {
                r[i][j] = self.m[i][0] * rhs.m[0][j]
                    + self.m[i][1] * rhs.m[1][j]
                    + self.m[i][2] * rhs.m[2][j]
                    + self.m[i][3] * rhs.m[3][j];
            }
        }
        return Matrix4x4::from(r);
    }
}

/// Matrix4x4 * &Point -> Point
impl Mul<&Point> for &Matrix4x4 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        let (x, y, z) = (rhs.x, rhs.y, rhs.z);
        let p = Point::new(
            self.m[0][0] * x + self.m[0][1] * y + self.m[0][2] * z + self.m[0][3],
            self.m[1][0] * x + self.m[1][1] * y + self.m[1][2] * z + self.m[1][3],
            self.m[2][0] * x + self.m[2][1] * y + self.m[2][2] * z + self.m[2][3],
        );
        let w = self.m[3][0] * x + self.m[3][1] * y + self.m[3][2] * z + self.m[3][3];
        return if w != 1.0 { p / w } else { p };
    }
}

/// Matrix4x4 * &Vector -> Vector
impl Mul<&Vector> for &Matrix4x4 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        let (x, y, z) = (rhs.x, rhs.y, rhs.z);
        Vector::new(
            self.m[0][0] * x + self.m[0][1] * y + self.m[0][2] * z,
            self.m[1][0] * x + self.m[1][1] * y + self.m[1][2] * z,
            self.m[2][0] * x + self.m[2][1] * y + self.m[2][2] * z,
        )
    }
}

/// Matrix4x4 * &Normal -> Normal
impl Mul<&Normal> for &Matrix4x4 {
    type Output = Normal;

    fn mul(self, rhs: &Normal) -> Self::Output {
        let (x, y, z) = (rhs.x, rhs.y, rhs.z);
        Normal::new(
            self.m[0][0] * x + self.m[0][1] * y + self.m[0][2] * z,
            self.m[1][0] * x + self.m[1][1] * y + self.m[1][2] * z,
            self.m[2][0] * x + self.m[2][1] * y + self.m[2][2] * z,
        )
    }
}

impl Mul<&Ray> for &Matrix4x4 {
    type Output = Ray;

    fn mul(self, rhs: &Ray) -> Self::Output {
        Ray::builder()
            .origin(self * &rhs.origin)
            .direction(self * &rhs.direction)
            .start(rhs.start)
            .end(rhs.end)
            .time(rhs.time)
            .build()
    }
}
