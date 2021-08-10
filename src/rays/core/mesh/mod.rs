pub use self::ext::{ExtMesh, ExtTriangleMesh};
use crate::rays::geometry::{Bounds, Point, Transform, Triangle};
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
pub mod triangle;

pub enum MeshType {
    Triangle,
    TriangleInstance,
    TriangleMotion,
    ExtTriangle,
    ExtTriangleInstance,
    ExtTriangleMotion,
}

pub trait Mesh: NamedObject {
    fn get_type(&self) -> MeshType;

    fn get_bounds(&self) -> Bounds;
    fn get_local2world(&self, time: f32, local2world: &mut Transform);
    fn get_vertex(&self, local2world: &Transform, vert_index: usize) -> Point;

    fn get_vertices(&self) -> &Vec<Point>;
    fn get_triangles(&self) -> &Vec<Triangle>;
    fn get_total_vertex_count(&self) -> usize;
    fn get_total_triangle_count(&self) -> usize;

    // This can be a very expansive function to run
    fn get_mesh_area(&mut self, local2world: &Transform) -> f32;
    fn get_triangle_area(&self, local2world: &Transform, tri_index: usize) -> f32;
    fn sample(
        &self,
        local2world: &Transform,
        tri_index: usize,
        u0: f32,
        u1: f32,
        p: &mut Point,
        b0: f32,
        b1: f32,
        b2: f32,
    );
    fn apply_transform(&mut self, trans: &Transform);
}
