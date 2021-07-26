pub use self::ext::{ExtMesh, ExtTriangleMesh};
use crate::rays::object::NamedObject;

///
/// * The inheritance scheme used here:
/// *
/// * | =>    TriangleMesh      => |
/// * Mesh => |                            | => ExtTriangleMesh
/// * | =>       ExtMesh        => |
/// *
/// * | => InstanceTriangleMesh => |
/// * Mesh => |                            | => ExtInstanceTriangleMesh
/// * | =>       ExtMesh        => |
/// *
/// * | => MotionTriangleMesh   => |
/// * Mesh => |                            | => ExtMotionTriangleMesh
/// * | =>       ExtMesh        => |
pub mod ext;

pub trait Mesh: NamedObject {
    fn get_total_triangle_count(&self) -> u32;
}
