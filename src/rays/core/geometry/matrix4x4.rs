use std::mem::swap;
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

    pub fn transpose(&self) -> Self {
        Matrix4x4::from([
            [self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0]],
            [self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1]],
            [self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2]],
            [self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]],
        ])
    }

    pub fn determinant(&self) -> f32 {
        // row expansion along the last row
        // for most matrices this would be most efficient
        let mut result = 0.0_f32;
        let mut s = -1.0_f32;
        let mut a: [[f32; 3]; 3] = Default::default();

        // initialize for first expansion
        for i in 0..3 {
            for j in 0..3 {
                a[i][j] = self.m[i][j + 1]
            }
        }

        let mut k = 0;

        loop {
            if self.m[3][k] != 0.0 {
                result += s * self.m[3][k] * det3x3(a);
            }
            // we're done
            if k >= 3 {
                break;
            }

            s *= -1.0_f32;

            // copy column for next expansion
            for i in 0..3 {
                a[i][k] = self.m[i][k];
            }
            k += 1;
        }

        return result;
    }

    pub fn inverse(&self) -> Self {
        let mut indxc: [i32; 4] = Default::default();
        let mut indxr: [i32; 4] = Default::default();

        let mut ipiv: [i32; 4] = Default::default();
        let mut minv: [[f32; 4]; 4] = self.m.clone();

        for i in 0..4 {
            let (mut irow, mut icol): (i32, i32) = (-1, -1);
            let mut big = 0.0_f32;

            // Choose pivot
            for j in 0..4 {
                if ipiv[j] != 1 {
                    for k in 0..4 {
                        if ipiv[k] == 0 {
                            if minv[j][k].abs() >= big {
                                big = minv[j][k].abs();
                                irow = j as i32;
                                icol = k as i32;
                            }
                        } else if ipiv[k] > 1 {
                            panic!("Singular matrix in MatrixInvert: {}", self.to_string())
                        }
                    }
                }
            }

            ipiv[icol as usize] += 1;

            // Swap rows _irow_ and _icol_ for pivot
            if irow != icol {
                for k in 0..4 {
                    swap(&mut minv[irow as usize][k], &mut minv[icol as usize][k]);
                }
            }
            indxr[i] = irow;
            indxc[i] = icol;
            if minv[icol as usize][icol as usize] == 0.0 {
                panic!("Singular matrix in MatrixInvert: {}", self.to_string());
            }

            // Set $m[icol][icol]$ to one by scaling row _icol_ appropriately
            let pivinv = 1.0_f32 / minv[icol as usize][icol as usize];
            minv[icol as usize][icol as usize] = 1.0_f32;
            for j in 0..4 {
                minv[icol as usize][j] *= pivinv;
            }
            // Subtract this row from others to zero out their columns
            for j in 0..4 {
                if j != icol {
                    let save: f32 = minv[j as usize][icol as usize];
                    minv[j as usize][icol as usize] = 0.0;
                    for k in 0..4 {
                        minv[j as usize][k] -= minv[icol as usize][k] * save;
                    }
                }
            }
        }
        // Swap columns to reflect permutation
        for j in (0..=3).rev() {
            if indxr[j] != indxc[j] {
                for k in 0..4 {
                    swap(
                        &mut minv[k][indxr[j] as usize],
                        &mut minv[k][indxc[j] as usize],
                    );
                }
            }
        }

        return Matrix4x4::from(minv);
    }
}

fn det2x2(a00: f32, a01: f32, a10: f32, a11: f32) -> f32 { a00 * a11 - a01 * a10 }

fn det3x3(a: [[f32; 3]; 3]) -> f32 {
    a[0][0] * det2x2(a[1][1], a[1][2], a[2][1], a[2][2])
        - a[0][1] * det2x2(a[1][0], a[1][2], a[2][0], a[2][2])
        + a[0][2] * det2x2(a[1][0], a[1][1], a[2][0], a[2][1])
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

impl ToString for Matrix4x4 {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for i in 0..4 {
            for j in 0..4 {
                if i != 0 || j != 0 {
                    string.push_str(" ");
                } else {
                    string.push_str(self.m[j][i].to_string().as_str());
                }
            }
        }
        return string;
    }
}

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
