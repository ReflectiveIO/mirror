use crate::rays::color::Spectrum;
use crate::rays::geometry::{Normal, Point, Triangle, UV};
use crate::rays::mesh::ExtTriangleMesh;
use crate::rays::utils::HairFile;
use crate::slg::shape::{Shape, ShapeType};
use crate::slg::Scene;

/// Types of strands tessellation.
pub enum TessellationType {
    // This list must be aligned with slg::shape::TessellationType
    Ribbon,
    RibbonAdaptive,
    Solid,
    SolidAdaptive,
}

pub struct StrandsShape {
    // Tessellation options
    adaptive_max_depth: usize,
    adaptive_error: f32,
    solid_side_count: usize,
    solid_cap_bottom: bool,
    solid_cap_top: bool,
    use_camera_position: bool,
    mesh: ExtTriangleMesh,
}

impl StrandsShape {
    pub fn new(
        scene: &Scene,
        hair_file: &HairFile,
        tessel_type: TessellationType,
        adaptive_max_depth: usize,
        adaptive_error: f32,
        solid_side_count: usize,
        solid_cap_bottom: bool,
        solid_cap_top: bool,
        use_camera_position: bool,
    ) -> Self {
        todo!()
    }

    pub fn tessellate_ribbon(
        &self,
        scene: &Scene,
        hair_points: &Vec<Point>,
        hair_sizes: &Vec<f32>,
        hair_cols: &Vec<Spectrum>,
        hair_uvs: &Vec<UV>,
        hair_transps: &Vec<f32>,
        mesh_verts: &Vec<Point>,
        mesh_norms: &Vec<Normal>,
        mesh_tris: &Vec<Triangle>,
        mesh_uvs: &Vec<UV>,
        mesh_cols: &Vec<Spectrum>,
        mesh_transps: &Vec<f32>,
    ) {
        todo!()
    }

    pub fn tessellate_adaptive(
        &self,
        solid: bool,
        scene: &Scene,
        hair_points: &Vec<Point>,
        hair_sizes: &Vec<f32>,
        hair_cols: &Vec<Spectrum>,
        hair_uvs: &Vec<UV>,
        hair_transps: &Vec<f32>,
        mesh_verts: &Vec<Point>,
        mesh_norms: &Vec<Normal>,
        mesh_tris: &Vec<Triangle>,
        mesh_uvs: &Vec<UV>,
        mesh_cols: &Vec<Spectrum>,
        mesh_transps: &Vec<f32>,
    ) {
        todo!()
    }

    pub fn tessellate_solid(
        &self,
        scene: &Scene,
        hair_points: &Vec<Point>,
        hair_sizes: &Vec<f32>,
        hair_cols: &Vec<Spectrum>,
        hair_uvs: &Vec<UV>,
        hair_transps: &Vec<f32>,
        mesh_verts: &Vec<Point>,
        mesh_norms: &Vec<Normal>,
        mesh_tris: &Vec<Triangle>,
        mesh_uvs: &Vec<UV>,
        mesh_cols: &Vec<Spectrum>,
        mesh_transps: &Vec<f32>,
    ) {
        todo!()
    }
}

impl Shape for StrandsShape {
    fn get_type(&self) -> ShapeType { ShapeType::Strands }

    fn refine(&self, scene: &Scene) -> ExtTriangleMesh { todo!() }
}
