use crate::rays::geometry::Matrix4x4;

/// Transform Declarations
#[derive(Clone, Debug, Default)]
pub struct Transform {
    pub m: Matrix4x4,
}
