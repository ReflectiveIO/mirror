use crate::rays::core::geometry::{Bounds, Point, Transform, Triangle};
use crate::rays::core::mesh::MeshType;
use crate::rays::core::named_object::NamedObject;
use crate::rays::mesh::Mesh;

pub struct TriangleMesh {
    vertices: Vec<Point>,
    triangles: Vec<Triangle>,
    area: f32,

    // The transformation that was applied to the vertices
    // (needed e.g. for LocalMapping3D evaluation)
    applied_trans: Transform,
    applied_trans_swaps_handedness: bool,

    cached_bounds: Bounds,
    cached_bounds_valid: bool,
}

impl NamedObject for TriangleMesh {
    fn get_name(&self) -> String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}

impl Mesh for TriangleMesh {
    fn get_type(&self) -> MeshType { MeshType::Triangle }

    fn get_bounds(&self) -> Bounds { todo!() }

    fn get_local2world(&self, time: f32, local2world: &mut Transform) {
        *local2world = self.applied_trans.clone();
    }

    fn get_vertex(&self, local2world: &Transform, vert_index: usize) -> Point {
        self.vertices[vert_index].clone()
    }

    fn get_vertices(&self) -> &Vec<Point> { &self.vertices }

    fn get_triangles(&self) -> &Vec<Triangle> { &self.triangles }

    fn get_total_vertex_count(&self) -> usize { self.vertices.len() }

    fn get_total_triangle_count(&self) -> usize { self.triangles.len() }

    fn get_mesh_area(&mut self, local2world: &Transform) -> f32 { self.area }

    fn get_triangle_area(&self, local2world: &Transform, tri_index: usize) -> f32 {
        self.triangles[tri_index].area(&self.vertices)
    }

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
    ) {
        let triangle = &self.triangles[tri_index];
        triangle.sample(&self.vertices, u0, u1, p, b0, b1, b2);
    }

    fn apply_transform(&mut self, trans: &Transform) { todo!() }
}
