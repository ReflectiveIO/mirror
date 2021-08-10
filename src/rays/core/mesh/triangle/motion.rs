use crate::rays::core::geometry::{Point, Triangle};
use crate::rays::core::mesh::MeshType;
use crate::rays::core::named_object::NamedObject;
use crate::rays::geometry::{area, Bounds, MotionSystem, Transform};
use crate::rays::mesh::triangle::TriangleMesh;
use crate::rays::mesh::Mesh;

pub struct MotionTriangleMesh {
    motion_system: MotionSystem,
    mesh: TriangleMesh,

    cached_area: f32,
    cached_bounds: Bounds,
    cached_bounds_valid: bool,
}

impl MotionTriangleMesh {
    pub fn new(mesh: &TriangleMesh, ms: &MotionSystem) -> Self { todo!() }

    pub fn get_triangle_mesh(&self) -> &TriangleMesh { &self.mesh }

    pub fn get_motion_system(&self) -> &MotionSystem { &self.motion_system }
}

impl NamedObject for MotionTriangleMesh {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}

impl Mesh for MotionTriangleMesh {
    fn get_type(&self) -> MeshType { MeshType::TriangleMotion }

    fn get_bounds(&self) -> Bounds { todo!() }

    fn get_local2world(&self, time: f32, local2world: &mut Transform) {
        let m = self.motion_system.sample_inverse(time);
        *local2world = Transform::from(m);
    }

    fn get_vertex(&self, local2world: &Transform, vert_index: usize) -> Point {
        local2world * self.mesh.get_vertex(local2world, vert_index)
    }

    fn get_vertices(&self) -> &Vec<Point> { &self.mesh.get_vertices() }

    fn get_triangles(&self) -> &Vec<Triangle> { &self.mesh.get_triangles() }

    fn get_total_vertex_count(&self) -> usize { self.mesh.get_total_vertex_count() }

    fn get_total_triangle_count(&self) -> usize { self.mesh.get_total_triangle_count() }

    fn get_mesh_area(&mut self, local2world: &Transform) -> f32 {
        if self.cached_area < 0.0 {
            let mut area = 0.0;
            for i in 0..self.get_total_triangle_count() {
                area += self.get_triangle_area(local2world, i);
            }
            // Cache the result
            self.cached_area = area;
        }
        self.cached_area
    }

    fn get_triangle_area(&self, local2world: &Transform, tri_index: usize) -> f32 {
        let triangle = &self.mesh.get_triangles()[tri_index];
        area(
            &self.get_vertex(local2world, triangle.v[0]),
            &self.get_vertex(local2world, triangle.v[1]),
            &self.get_vertex(local2world, triangle.v[2]),
        )
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
        self.mesh
            .sample(local2world, tri_index, u0, u1, p, b0, b1, b2);
        *p = local2world * *p;
    }

    fn apply_transform(&mut self, trans: &Transform) { todo!() }
}
