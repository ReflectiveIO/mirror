use crate::rays::mesh::ExtTriangleMesh;
use crate::slg::Scene;

pub enum ShapeType {
    Mesh,
    Pointiness,
    Strands,
    Group,
    Subdiv,
    Displacement,
    Harlequin,
    Simplify,
    IslandAOV,
    RandomTriangleAOV,
    EdgeDetectorAOV,
}

pub trait Shape {
    fn get_type(&self) -> ShapeType;

    /// Note: this method can be called only once and the object is not usable
    /// anymore (this is mostly due to optimize memory management)
    fn refine(&self, scene: &Scene) -> ExtTriangleMesh;
}
