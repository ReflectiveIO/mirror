pub use self::ext::ExtMesh;
pub use self::ext::ExtTriangleMesh;

///
/// * The inheritance scheme used here:
/// *
/// *         | =>    TriangleMesh      => |
/// * Mesh => |                            | => ExtTriangleMesh
/// *         | =>       ExtMesh        => |
/// *
/// *         | => InstanceTriangleMesh => |
/// * Mesh => |                            | => ExtInstanceTriangleMesh
/// *         | =>       ExtMesh        => |
/// *
/// *         | => MotionTriangleMesh   => |
/// * Mesh => |                            | => ExtMotionTriangleMesh
/// *         | =>       ExtMesh        => |
///
pub mod ext;
