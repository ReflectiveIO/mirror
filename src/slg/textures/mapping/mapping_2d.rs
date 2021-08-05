use std::str::FromStr;

use crate::rays::geometry::UV;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::textures::RandomMappingSeedType;

/// TextureMapping2DType
pub enum TextureMapping2DType {
    UvMapping,
    UvRandomMapping,
}

impl FromStr for TextureMapping2DType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> { todo!() }
}

impl ToString for TextureMapping2DType {
    fn to_string(&self) -> String { todo!() }
}

/// TextureMapping2D
pub trait TextureMapping2D {
    fn get_data_index(&self) -> u32;
    fn get_type(&self) -> TextureMapping2DType;
    fn map(&self, hp: &HitPoint) -> UV;
    fn map_duv(&self, hp: &HitPoint, ds: &UV, dt: &UV) -> UV;
    fn to_properties(&self, name: &String) -> Properties;
}

/// UVMapping2D
pub struct UVMapping2D {
    data_index: u32,

    pub uv_rotation: f32,
    pub u_scale: f32,
    pub v_scale: f32,
    pub u_delta: f32,
    pub v_delta: f32,
    pub sin_theta: f32,
    pub cos_theta: f32,
}

impl UVMapping2D {
    pub fn new(
        data_index: u32,
        uv_rotation: f32,
        u_scale: f32,
        v_scale: f32,
        u_delta: f32,
        v_delta: f32,
    ) -> Self {
        Self {
            data_index,
            uv_rotation,
            u_scale,
            v_scale,
            u_delta,
            v_delta,
            sin_theta: 0.0,
            cos_theta: 0.0,
        }
    }
}

impl TextureMapping2D for UVMapping2D {
    fn get_data_index(&self) -> u32 { self.data_index }

    fn get_type(&self) -> TextureMapping2DType { TextureMapping2DType::UvMapping }

    fn map(&self, hp: &HitPoint) -> UV { todo!() }

    fn map_duv(&self, hp: &HitPoint, ds: &UV, dt: &UV) -> UV { todo!() }

    fn to_properties(&self, name: &String) -> Properties { todo!() }
}

/// UVRandomMapping2D
pub struct UVRandomMapping2D {
    data_index: u32,

    pub seed_type: RandomMappingSeedType,
    pub tri_aov_index: u32,
    pub object_id_offset: u32,
    pub uv_rotation_min: f32,
    pub uv_rotation_max: f32,
    pub u_scale_min: f32,
    pub u_scale_max: f32,
    pub v_scale_min: f32,
    pub v_scale_max: f32,
    pub u_delta_min: f32,
    pub u_delta_max: f32,
    pub v_delta_min: f32,
    pub v_delta_max: f32,
    pub uniform_scale: f32,
}

impl UVRandomMapping2D {
    pub fn new(
        data_index: u32,

        seed_type: RandomMappingSeedType,
        tri_aov_index: u32,
        object_id_offset: u32,
        uv_rotation_min: f32,
        uv_rotation_max: f32,
        u_scale_min: f32,
        u_scale_max: f32,
        v_scale_min: f32,
        v_scale_max: f32,
        u_delta_min: f32,
        u_delta_max: f32,
        v_delta_min: f32,
        v_delta_max: f32,
        uniform_scale: f32,
    ) -> Self {
        Self {
            data_index,
            seed_type,
            tri_aov_index,
            object_id_offset,
            uv_rotation_min,
            uv_rotation_max,
            u_scale_min,
            u_scale_max,
            v_scale_min,
            v_scale_max,
            u_delta_min,
            u_delta_max,
            v_delta_min,
            v_delta_max,
            uniform_scale,
        }
    }
}

impl TextureMapping2D for UVRandomMapping2D {
    fn get_data_index(&self) -> u32 { self.data_index }

    fn get_type(&self) -> TextureMapping2DType { TextureMapping2DType::UvRandomMapping }

    fn map(&self, hp: &HitPoint) -> UV { todo!() }

    fn map_duv(&self, hp: &HitPoint, ds: &UV, dt: &UV) -> UV { todo!() }

    fn to_properties(&self, name: &String) -> Properties { todo!() }
}
