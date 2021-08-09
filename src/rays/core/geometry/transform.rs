use std::ops::{Div, Mul};

use crate::rays::geometry::{Cross, Matrix4x4, Point, Vector};

pub struct InvTransform {
    reference: Transform,
}

impl From<Transform> for InvTransform {
    fn from(t: Transform) -> Self { Self { reference: t } }
}

/// Transform Declarations
#[derive(Clone, Debug, Default)]
pub struct Transform {
    pub m: Matrix4x4,
    pub m_inv: Matrix4x4,
}

impl Transform {
    pub(crate) fn inverse(&self) -> Transform { todo!() }
}

impl Transform {
    const TRANS_IDENTITY: Transform = Transform::new();

    pub fn new() -> Self {
        Self {
            m: Matrix4x4::IDENTITY,
            m_inv: Matrix4x4::IDENTITY,
        }
    }

    pub fn get_matrix(&self) -> &Matrix4x4 { &self.m }

    pub fn has_scale(&self) -> bool {
        let det = (self.m.m[0][0]
            * (self.m.m[1][1] * self.m.m[2][2] - self.m.m[1][2] * self.m.m[2][1]))
            .abs()
            - (self.m.m[0][1]
                * (self.m.m[1][0] * self.m.m[2][2] - self.m.m[1][2] * self.m.m[2][0]))
            + (self.m.m[0][2]
                * (self.m.m[1][0] * self.m.m[2][1] - self.m.m[1][1] * self.m.m[2][0]));
        return det < 0.999 || det > 1.001;
    }

    #[inline]
    pub fn swaps_handedness(&self) -> bool {
        let det = self.m.m[0][0]
            * (self.m.m[1][1] * self.m.m[2][2] - self.m.m[1][2] * self.m.m[2][1])
            - (self.m.m[0][1]
                * (self.m.m[1][0] * self.m.m[2][2] - self.m.m[1][2] * self.m.m[2][0]))
            + (self.m.m[0][2]
                * (self.m.m[1][0] * self.m.m[2][1] - self.m.m[1][1] * self.m.m[2][0]));
        return det < 0.0;
    }
}

impl From<[[f32; 4]; 4]> for Transform {
    fn from(mat: [[f32; 4]; 4]) -> Self {
        let matrix = Matrix4x4::from(mat);
        Self {
            m: matrix,
            m_inv: matrix.inverse(),
        }
    }
}

impl From<Matrix4x4> for Transform {
    fn from(mat: Matrix4x4) -> Self {
        Self {
            m: mat,
            m_inv: mat.inverse(),
        }
    }
}

impl From<(Matrix4x4, Matrix4x4)> for Transform {
    fn from((m, m_inv): (Matrix4x4, Matrix4x4)) -> Self { Self { m, m_inv } }
}

impl From<InvTransform> for Transform {
    fn from(t: InvTransform) -> Self {
        Self {
            m: t.reference.m_inv,
            m_inv: t.reference.m,
        }
    }
}

impl Mul for Transform {
    type Output = Transform;

    fn mul(self, rhs: Self) -> Self::Output {
        Transform::from((self.m * &rhs.m, rhs.m_inv * &self.m_inv))
    }
}

impl Mul<Vector> for Transform {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output { todo!() }
}

impl Div for Transform {
    type Output = Transform;

    fn div(self, rhs: Self) -> Self::Output {
        Transform::from((self.m * &rhs.m_inv, rhs.m * &self.m_inv))
    }
}

#[inline]
pub fn inverse(t: Transform) -> InvTransform { InvTransform::from(t) }

pub fn translate(delta: &Vector) -> Transform { todo!() }
pub fn scale(x: f32, y: f32, z: f32) -> Transform { todo!() }
pub fn rotate_x(angle: f32) -> Transform { todo!() }
pub fn rotate_y(angle: f32) -> Transform { todo!() }
pub fn rotate_z(angle: f32) -> Transform { todo!() }
pub fn rotate(angle: f32, axis: &Vector) -> Transform { todo!() }

pub fn look_at(pos: &Point, look: &Point, up: &Vector) -> Transform {
    let mut m: [[f32; 4]; 4] = Default::default();

    // Initialize fourth column of viewing matrix
    m[0][3] = pos.x;
    m[1][3] = pos.y;
    m[2][3] = pos.z;
    m[3][3] = 1.0;

    let dir = Vector::from(*look - *pos).normalize();
    let right = dir.cross(up).normalize();
    let up = right.cross(&dir);

    m[0][0] = right.x;
    m[1][0] = right.y;
    m[2][0] = right.z;
    m[3][0] = 0.;
    m[0][1] = up.x;
    m[1][1] = up.y;
    m[2][1] = up.z;
    m[3][1] = 0.;
    m[0][2] = dir.x;
    m[1][2] = dir.y;
    m[2][2] = dir.z;
    m[3][2] = 0.0;

    let cam_to_world = Matrix4x4::from(m);
    return Transform::from((cam_to_world.inverse(), cam_to_world));
}

pub fn orthographic(near: f32, far: f32) -> Transform { todo!() }
pub fn perspective(ov: f32, near: f32, far: f32) -> Transform { todo!() }
