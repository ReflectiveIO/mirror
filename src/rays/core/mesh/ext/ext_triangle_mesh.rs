use crate::rays::color::Spectrum;
use crate::rays::geometry::{Normal, Point, Triangle, UV};
use crate::rays::mesh::{ExtMesh, Mesh};
use crate::rays::object::NamedObject;

#[derive(Default)]
pub struct ExtTriangleMesh;

impl ExtTriangleMesh {
    pub fn new(
        ply_nb_verts: i64,
        ply_nb_tris: i64,
        p: Point,
        vi: Triangle,
        n: Normal,
        uvs: Vec<UV>,
        cols: Vec<Box<Spectrum>>,
        alphas: Vec<Vec<f32>>,
    ) -> Self {
        todo!()
    }
}

impl Mesh for ExtTriangleMesh {
    fn get_total_triangle_count(&self) -> u32 { todo!() }
}

impl ExtMesh for ExtTriangleMesh {
    fn get_tri_aov(&self, triangle_index: u32, data_index: u32) -> f32 { todo!() }

    fn interpolate_tri_uv(&self, triangle_index: u32, b1: f32, b2: f32, data_index: u32) -> UV {
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
}

impl NamedObject for ExtTriangleMesh {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
