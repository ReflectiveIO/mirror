use crate::rays::geometry::Transform;
use crate::rays::mesh::ExtTriangleMesh;
use crate::slg::shape::{Shape, ShapeType};
use crate::slg::Scene;

pub struct MeshShape {
    mesh: ExtTriangleMesh,
}

impl MeshShape {
    pub fn new(mesh: &ExtTriangleMesh) -> Self { todo!() }

    pub fn load(filename: &String) -> Self { todo!() }

    pub fn set_local2world(&mut self, trans: &Transform) { todo!() }

    pub fn apply_transform(&mut self, trans: &Transform) { todo!() }
}

impl Shape for MeshShape {
    fn get_type(&self) -> ShapeType { ShapeType::Mesh }

    fn refine(&self, scene: &Scene) -> ExtTriangleMesh { todo!() }
}
