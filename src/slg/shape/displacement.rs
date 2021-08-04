use crate::rays::mesh::ExtTriangleMesh;
use crate::slg::shape::{Shape, ShapeType};
use crate::slg::textures::Texture;
use crate::slg::Scene;

pub struct DisplacementShape {
    mesh: ExtTriangleMesh,
}

pub enum DisplacementType {
    HightDisplacement,
    VectorDisplacement,
}

pub struct Params {
    map_type: DisplacementType,
    map_channels: [usize; 3],
    scale: f32,
    offset: f32,
    uv_index: usize,
    normal_smooth: bool,
}

impl DisplacementShape {
    pub fn new(src: &ExtTriangleMesh, disp: &Box<dyn Texture>, params: &Params) -> Self { todo!() }
}

impl Shape for DisplacementShape {
    fn get_type(&self) -> ShapeType { ShapeType::Displacement }

    fn refine(&self, scene: &Scene) -> ExtTriangleMesh { todo!() }
}
