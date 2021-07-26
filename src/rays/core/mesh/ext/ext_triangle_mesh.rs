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

impl ExtMesh for ExtTriangleMesh {}

impl NamedObject for ExtTriangleMesh {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
