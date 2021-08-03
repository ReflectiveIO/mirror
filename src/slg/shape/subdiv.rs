use crate::rays::mesh::ExtTriangleMesh;
use crate::slg::cameras::Camera;
use crate::slg::shape::{Shape, ShapeType};
use crate::slg::Scene;

pub struct SubdivShape {
    mesh: ExtTriangleMesh,
}

impl SubdivShape {
    pub fn new(
        camera: &dyn Camera,
        src: &ExtTriangleMesh,
        max_level: usize,
        max_edge_screen_size: f32,
    ) -> Self {
        todo!()
    }

    pub fn max_edge_screen_size(camera: &dyn Camera, src: &ExtTriangleMesh) -> f32 { todo!() }

    pub fn apply_subdiv(src: &ExtTriangleMesh, max_level: usize) -> ExtTriangleMesh { todo!() }
}

impl Shape for SubdivShape {
    fn get_type(&self) -> ShapeType { ShapeType::Subdiv }

    fn refine(&self, scene: &Scene) -> ExtTriangleMesh { todo!() }
}
