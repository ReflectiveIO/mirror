pub use self::ext::{ExtMesh, ExtTriangleMesh};

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

pub trait Mesh {
    fn get_total_triangle_count(&self) -> u32;
}
