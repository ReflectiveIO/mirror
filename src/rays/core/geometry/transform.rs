use crate::rays::geometry::{Matrix4x4, Point, Vector};

pub type Transform = na::Transform3<f32>;

/// Builds a translation 4 * 4 matrix created from a vector of 3 components
pub fn translate(delta: &Vector) -> Transform {
    Transform::from_matrix_unchecked(Matrix4x4::identity().prepend_translation(delta))
}

/// Builds a scale 4 * 4 matrix created from 3 scalars

pub fn scale(x: f32, y: f32, z: f32) -> Transform {
    Transform::from_matrix_unchecked(
        Matrix4x4::identity().prepend_nonuniform_scaling(&Vector::new(x, y, z)),
    )
}

/// Build a look at view matrix based on the right handedness.
pub fn look_at(eye: &Point, center: &Point, up: &Vector) -> Transform {
    Transform::from_matrix_unchecked(Matrix4x4::look_at_rh(eye, center, up))
}

/// Builds a rotation 4 * 4 matrix created from an axis vector and an angle
pub fn rotate(angle: f32, axis: &Vector) -> Transform {
    Transform::from_matrix_unchecked(
        Matrix4x4::identity()
            * na::Rotation3::from_axis_angle(&na::Unit::new_normalize(*axis), angle)
                .to_homogeneous(),
    )
}

/// Builds a rotation 4 * 4 matrix around the X axis
pub fn rotate_x(angle: f32) -> Transform { rotate(angle, &Vector::x()) }

/// Builds a rotation 4 * 4 matrix around the Y axis
pub fn rotate_y(angle: f32) -> Transform { rotate(angle, &Vector::y()) }

/// Builds a rotation 4 * 4 matrix around the Z axis
pub fn rotate_z(angle: f32) -> Transform { rotate(angle, &Vector::z()) }
