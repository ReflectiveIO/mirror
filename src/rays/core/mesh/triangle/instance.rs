use crate::rays::core::geometry::{Point, Triangle};
use crate::rays::core::mesh::MeshType;
use crate::rays::core::named_object::NamedObject;
use crate::rays::geometry::{area, Bounds, Transform};
use crate::rays::mesh::triangle::TriangleMesh;
use crate::rays::mesh::Mesh;

pub struct InstanceTriangleMesh {
    trans: Transform,
    trans_swaps_handedness: bool,
    mesh: TriangleMesh,

    cached_area: f32,
    cached_bounds: Bounds,
    cached_bounds_valid: bool,
}

impl InstanceTriangleMesh {
    pub fn new(mesh: &TriangleMesh, trans: &Transform) -> Self { todo!() }

    pub fn get_triangle_mesh(&self) -> &TriangleMesh { &self.mesh }
}

impl NamedObject for InstanceTriangleMesh {
    fn get_name(&self) -> String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}

impl Mesh for InstanceTriangleMesh {
    fn get_type(&self) -> MeshType { MeshType::TriangleInstance }

    fn get_bounds(&self) -> Bounds { todo!() }

    fn get_local2world(&self, time: f32, local2world: &mut Transform) {
        *local2world = self.trans.clone();
    }

    fn get_vertex(&self, local2world: &Transform, vert_index: usize) -> Point {
        &self.trans * self.mesh.get_vertex(local2world, vert_index)
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
        *p *= &self.trans
    }

    fn apply_transform(&mut self, trans: &Transform) {
        self.trans = &self.trans * trans;
        self.trans_swaps_handedness = trans.swaps_handedness();
        self.cached_bounds_valid = false;

        // Invalidate the cached result
        self.cached_area = -1.0;
    }
}
