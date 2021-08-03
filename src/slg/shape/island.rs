use crate::rays::mesh::ExtTriangleMesh;
use crate::slg::shape::{Shape, ShapeType};
use crate::slg::Scene;

pub struct IslandAOVShape {
    mesh: ExtTriangleMesh,
}

impl IslandAOVShape {
    pub fn new(src: &ExtTriangleMesh, data_index: usize) -> Self { todo!() }
}

impl Shape for IslandAOVShape {
    fn get_type(&self) -> ShapeType { ShapeType::IslandAOV }

    fn refine(&self, scene: &Scene) -> ExtTriangleMesh { todo!() }
}
