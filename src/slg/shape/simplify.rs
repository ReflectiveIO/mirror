use crate::rays::mesh::ExtTriangleMesh;
use crate::slg::cameras::Camera;
use crate::slg::shape::{Shape, ShapeType};
use crate::slg::Scene;

pub struct SimplifyShape {
    mesh: ExtTriangleMesh,
}

impl SimplifyShape {
    pub fn new(
        camera: &dyn Camera,
        src: &ExtTriangleMesh,
        target: f32,
        edge_screen_size: f32,
        preserve_border: bool,
    ) -> Self {
        todo!()
    }
}

impl Shape for SimplifyShape {
    fn get_type(&self) -> ShapeType { ShapeType::Simplify }

    fn refine(&self, scene: &Scene) -> ExtTriangleMesh { todo!() }
}
