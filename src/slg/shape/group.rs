use crate::rays::geometry::Transform;
use crate::rays::mesh::ExtTriangleMesh;
use crate::slg::shape::{Shape, ShapeType};
use crate::slg::Scene;

pub struct GroupShape {
    meshes: Vec<ExtTriangleMesh>,
    trans: Vec<Transform>,
}

impl GroupShape {
    pub fn new(meshes: &Vec<ExtTriangleMesh>, trans: &Vec<Transform>) -> Self { todo!() }
}

impl Shape for GroupShape {
    fn get_type(&self) -> ShapeType { ShapeType::Group }

    fn refine(&self, scene: &Scene) -> ExtTriangleMesh { todo!() }
}
