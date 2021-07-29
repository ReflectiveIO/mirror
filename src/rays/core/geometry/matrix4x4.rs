use std::ops::Mul;

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
