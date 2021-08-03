use crate::rays::mesh::ExtTriangleMesh;
use crate::slg::shape::{Shape, ShapeType};
use crate::slg::Scene;

pub struct RandomTriangleAOVShape {
    mesh: ExtTriangleMesh,
}

impl RandomTriangleAOVShape {
    pub fn new(src: &ExtTriangleMesh, src_data_index: usize, dist_data_index: usize) -> Self {
        todo!()
    }
}

impl Shape for RandomTriangleAOVShape {
    fn get_type(&self) -> ShapeType { ShapeType::RandomTriangleAOV }

    fn refine(&self, scene: &Scene) -> ExtTriangleMesh { todo!() }
}
