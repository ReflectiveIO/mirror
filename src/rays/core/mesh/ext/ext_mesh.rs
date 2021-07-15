use crate::rays::mesh::Mesh;

#[derive(Default)]
pub struct ExtMesh;

impl Mesh for ExtMesh {
    fn get_total_triangle_count(&self) -> u32 { 0 }
}
