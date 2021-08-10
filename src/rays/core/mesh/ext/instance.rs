use crate::rays::color::Spectrum;
use crate::rays::core::geometry::{Bounds, Normal, Point, Transform, Triangle, Vector, UV};
use crate::rays::core::mesh::{Mesh, MeshType};
use crate::rays::core::named_object::NamedObject;
use crate::rays::mesh::ExtMesh;

pub struct ExtInstanceTriangleMesh;

impl Mesh for ExtInstanceTriangleMesh {
    fn get_type(&self) -> MeshType { MeshType::ExtTriangleInstance }

    fn get_bounds(&self) -> Bounds { todo!() }

    fn get_local2world(&self, time: f32, local2world: &mut Transform) { todo!() }

    fn get_vertex(&self, local2world: &Transform, vert_index: usize) -> Point { todo!() }

    fn get_vertices(&self) -> &Vec<Point> { todo!() }

    fn get_triangles(&self) -> &Vec<Triangle> { todo!() }

    fn get_total_vertex_count(&self) -> usize { todo!() }

    fn get_total_triangle_count(&self) -> usize { todo!() }

    fn get_mesh_area(&mut self, local2world: &Transform) -> f32 { todo!() }

    fn get_triangle_area(&self, local2world: &Transform, tri_index: usize) -> f32 { todo!() }

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
        todo!()
    }

    fn apply_transform(&mut self, trans: &Transform) { todo!() }
}

impl NamedObject for ExtInstanceTriangleMesh {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}

impl ExtMesh for ExtInstanceTriangleMesh {
    fn has_normals(&self) -> bool { todo!() }

    fn has_uvs(&self, data_index: u32) -> bool { todo!() }

    fn has_colors(&self, data_index: u32) -> bool { todo!() }

    fn has_alphas(&self, data_index: u32) -> bool { todo!() }

    fn has_vertex_aov(&self, data_index: u32) -> bool { todo!() }

    fn has_tri_aov(&self, data_index: u32) -> bool { todo!() }

    fn get_geometry_normal(&self, local2world: &Transform, tri_index: u32) -> Normal { todo!() }

    fn get_shade_normal(&self, local2world: &Transform, tri_index: u32, vert_index: u32) -> Normal {
        todo!()
    }

    fn get_shade_normal2(&self, local2world: &Transform, vert_index: u32) -> Normal { todo!() }

    fn get_uv(&self, vert_index: u32, data_index: u32) -> UV { todo!() }

    fn get_color(&self, vert_index: u32, data_index: u32) -> UV { todo!() }

    fn get_alpha(&self, vert_index: u32, data_index: u32) -> UV { todo!() }

    fn get_vertex_aov(&self, vert_index: u32, data_index: u32) -> UV { todo!() }

    fn get_tri_aov(&self, tri_index: u32, data_index: u32) -> f32 { todo!() }

    fn get_tri_bary_coords(
        &self,
        local2world: &Transform,
        tri_index: u32,
        hp: &Point,
        b1: f32,
        b2: f32,
    ) -> bool {
        todo!()
    }

    fn get_differentials(
        &self,
        local2world: &Transform,
        tri_index: u32,
        shade_normal: &Normal,
        data_index: u32,
        dpdu: &mut Vector,
        dpdv: &mut Vector,
        dndu: &mut Normal,
        dndv: &mut Normal,
    ) {
        todo!()
    }

    fn interpolate_tri_normal(
        &self,
        local2world: &Transform,
        tri_index: u32,
        b1: f32,
        b2: f32,
    ) -> Normal {
        todo!()
    }

    fn interpolate_tri_uv(&self, tri_index: u32, b1: f32, b2: f32, data_index: u32) -> UV {
        todo!()
    }

    fn interpolate_tri_color(
        &self,
        triangle_index: u32,
        b1: f32,
        b2: f32,
        data_index: u32,
    ) -> Spectrum {
        todo!()
    }

    fn interpolate_tri_alpha(&self, triangle_index: u32, b1: f32, b2: f32, data_index: u32) -> f32 {
        todo!()
    }

    fn interpolate_tri_vertex_aov(
        &self,
        triangle_index: u32,
        b1: f32,
        b2: f32,
        data_index: u32,
    ) -> f32 {
        todo!()
    }

    fn delete(&mut self) { todo!() }

    fn save(&self, filename: &str) { todo!() }
}
