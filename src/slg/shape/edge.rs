use crate::rays::mesh::ExtTriangleMesh;
use crate::slg::shape::{Shape, ShapeType};
use crate::slg::Scene;

pub struct EdgeDetectorAOVShape {
    mesh: ExtTriangleMesh,
}

impl EdgeDetectorAOVShape {
    pub fn new(
        src: &ExtTriangleMesh,
        dist_aov_index0: usize,
        dest_aov_index1: usize,
        dest_aov_index2: usize,
    ) -> Self {
        todo!()
    }
}

impl Shape for EdgeDetectorAOVShape {
    fn get_type(&self) -> ShapeType { ShapeType::EdgeDetectorAOV }

    fn refine(&self, scene: &Scene) -> ExtTriangleMesh { todo!() }
}
