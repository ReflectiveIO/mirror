use downcast_rs::Downcast;

use crate::rays::color::Spectrum;
use crate::rays::geometry::UV;
use crate::rays::mesh::Mesh;

pub trait ExtMesh: Mesh + Downcast {
    fn get_tri_aov(&self, triangle_index: u32, data_index: u32) -> f32;
    fn interpolate_tri_uv(&self, triangle_index: u32, b1: f32, b2: f32, data_index: u32) -> UV;
    fn interpolate_tri_color(
        &self,
        triangle_index: u32,
        b1: f32,
        b2: f32,
        data_index: u32,
    ) -> Spectrum;
    fn interpolate_tri_alpha(&self, triangle_index: u32, b1: f32, b2: f32, data_index: u32) -> f32;
    fn interpolate_tri_vertex_aov(
        &self,
        triangle_index: u32,
        b1: f32,
        b2: f32,
        data_index: u32,
    ) -> f32;
}
impl_downcast!(ExtMesh);
