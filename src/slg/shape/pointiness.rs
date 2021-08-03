use crate::rays::mesh::ExtTriangleMesh;
use crate::slg::shape::{Shape, ShapeType};
use crate::slg::Scene;

pub struct PointinessShape {
    mesh: ExtTriangleMesh,
}

impl PointinessShape {
    pub fn new(src: ExtTriangleMesh, dest_aov_index: usize) -> Self { todo!() }
}

impl Shape for PointinessShape {
    fn get_type(&self) -> ShapeType { ShapeType::Pointiness }

    fn refine(&self, scene: &Scene) -> ExtTriangleMesh { todo!() }
}
