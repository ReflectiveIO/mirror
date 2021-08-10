use downcast_rs::Downcast;

use crate::rays::color::Spectrum;
use crate::rays::geometry::{Normal, Point, Transform, Vector, UV};
use crate::rays::mesh::Mesh;

pub trait ExtMesh: Mesh + Downcast {
    fn has_normals(&self) -> bool;
    fn has_uvs(&self, data_index: u32) -> bool;
    fn has_colors(&self, data_index: u32) -> bool;
    fn has_alphas(&self, data_index: u32) -> bool;

    fn has_vertex_aov(&self, data_index: u32) -> bool;
    fn has_tri_aov(&self, data_index: u32) -> bool;

    fn get_geometry_normal(&self, local2world: &Transform, tri_index: u32) -> Normal;
    fn get_shade_normal(&self, local2world: &Transform, tri_index: u32, vert_index: u32) -> Normal;
    fn get_shade_normal2(&self, local2world: &Transform, vert_index: u32) -> Normal;

    fn get_uv(&self, vert_index: u32, data_index: u32) -> UV;
    fn get_color(&self, vert_index: u32, data_index: u32) -> UV;
    fn get_alpha(&self, vert_index: u32, data_index: u32) -> UV;

    fn get_vertex_aov(&self, vert_index: u32, data_index: u32) -> UV;
    fn get_tri_aov(&self, tri_index: u32, data_index: u32) -> f32;

    fn get_tri_bary_coords(
        &self,
        local2world: &Transform,
        tri_index: u32,
        hp: &Point,
        b1: f32,
        b2: f32,
    ) -> bool;
    fn get_differentials(
        &self,
        local2world: &Transform,
        tri_index: u32,
        shade_normal: &Normal,
        data_index: u32,
        dpdu: &mut Vector,
        dpdv: &mut Vector,
        dndu: &mut Normal,
        dndv: &mut Normal,
    );

    fn interpolate_tri_normal(
        &self,
        local2world: &Transform,
        tri_index: u32,
        b1: f32,
        b2: f32,
    ) -> Normal;
    fn interpolate_tri_uv(&self, tri_index: u32, b1: f32, b2: f32, data_index: u32) -> UV;
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

    fn delete(&mut self);
    fn save(&self, filename: &str);
}
impl_downcast!(ExtMesh);
