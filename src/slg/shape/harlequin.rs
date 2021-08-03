use crate::rays::mesh::ExtTriangleMesh;
use crate::slg::shape::{Shape, ShapeType};
use crate::slg::Scene;

pub struct HarlequinShape {
    mesh: ExtTriangleMesh,
}

impl HarlequinShape {
    pub fn new(src: &ExtTriangleMesh) -> Self { todo!() }
}

impl Shape for HarlequinShape {
    fn get_type(&self) -> ShapeType { ShapeType::Harlequin }

    fn refine(&self, scene: &Scene) -> ExtTriangleMesh { todo!() }
}
