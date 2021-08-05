use std::str::FromStr;

use crate::rays::geometry::{Point, Transform};
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::textures::{NormalMapTexture, RandomMappingSeedType};

/// TextureMapping3DType
pub enum TextureMapping3DType {
    UvMapping,
    GlobalMapping,
    LocalMapping,
    LocalRandomMapping,
}

impl FromStr for TextureMapping3DType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> { todo!() }
}

impl ToString for TextureMapping3DType {
    fn to_string(&self) -> String { todo!() }
}

/// TextureMapping3D
pub trait TextureMapping3D {
    fn get_type(&self) -> TextureMapping3DType;
    fn map(&self, hp: &HitPoint, shade_n: Option<NormalMapTexture>) -> Point;
    fn to_properties(&self, name: &String) -> Properties;
}

/// UVMapping3D
pub struct UVMapping3D {
    pub world2local: Transform,
    data_index: u32,
}

impl UVMapping3D {
    pub fn get_data_index(&self) -> u32 { self.data_index }
}

impl TextureMapping3D for UVMapping3D {
    fn get_type(&self) -> TextureMapping3DType { TextureMapping3DType::UvMapping }

    fn map(&self, hp: &HitPoint, shade_n: Option<NormalMapTexture>) -> Point { todo!() }

    fn to_properties(&self, name: &String) -> Properties { todo!() }
}

/// GlobalMapping3D
pub struct GlobalMapping3D {
    pub world2local: Transform,
}

impl TextureMapping3D for GlobalMapping3D {
    fn get_type(&self) -> TextureMapping3DType { TextureMapping3DType::GlobalMapping }

    fn map(&self, hp: &HitPoint, shade_n: Option<NormalMapTexture>) -> Point { todo!() }

    fn to_properties(&self, name: &String) -> Properties { todo!() }
}

/// LocalMapping3D
pub struct LocalMapping3D {
    pub world2local: Transform,
}

impl TextureMapping3D for LocalMapping3D {
    fn get_type(&self) -> TextureMapping3DType { TextureMapping3DType::LocalMapping }

    fn map(&self, hp: &HitPoint, shade_n: Option<NormalMapTexture>) -> Point { todo!() }

    fn to_properties(&self, name: &String) -> Properties { todo!() }
}

/// LocalRandomMapping3D
pub struct LocalRandomMapping3D {
    pub world2local: Transform,

    pub seed_type: RandomMappingSeedType,
    pub tri_aov_index: u32,
    pub object_id_offset: u32,
    pub x_rotation_min: f32,
    pub x_rotation_max: f32,
    pub y_rotation_min: f32,
    pub y_rotation_max: f32,
    pub z_rotation_min: f32,
    pub z_rotation_max: f32,
    pub x_scale_min: f32,
    pub x_scale_max: f32,
    pub y_scale_min: f32,
    pub y_scale_max: f32,
    pub z_scale_min: f32,
    pub z_scale_max: f32,
    pub x_translate_min: f32,
    pub x_translate_max: f32,
    pub y_translate_min: f32,
    pub y_translate_max: f32,
    pub z_translate_min: f32,
    pub z_translate_max: f32,

    pub uniform_scale: bool,
}

impl LocalRandomMapping3D {
    pub fn new(
        world2local: Transform,

        seed_type: RandomMappingSeedType,
        tri_aov_index: u32,
        object_id_offset: u32,
        x_rotation_min: f32,
        x_rotation_max: f32,
        y_rotation_min: f32,
        y_rotation_max: f32,
        z_rotation_min: f32,
        z_rotation_max: f32,
        x_scale_min: f32,
        x_scale_max: f32,
        y_scale_min: f32,
        y_scale_max: f32,
        z_scale_min: f32,
        z_scale_max: f32,
        x_translate_min: f32,
        x_translate_max: f32,
        y_translate_min: f32,
        y_translate_max: f32,
        z_translate_min: f32,
        z_translate_max: f32,

        uniform_scale: bool,
    ) -> Self {
        Self {
            world2local,
            seed_type,
            tri_aov_index,
            object_id_offset,
            x_rotation_min,
            x_rotation_max,
            y_rotation_min,
            y_rotation_max,
            z_rotation_min,
            z_rotation_max,
            x_scale_min,
            x_scale_max,
            y_scale_min,
            y_scale_max,
            z_scale_min,
            z_scale_max,
            x_translate_min,
            x_translate_max,
            y_translate_min,
            y_translate_max,
            z_translate_min,
            z_translate_max,
            uniform_scale,
        }
    }
}

impl TextureMapping3D for LocalRandomMapping3D {
    fn get_type(&self) -> TextureMapping3DType { TextureMapping3DType::LocalRandomMapping }

    fn map(&self, hp: &HitPoint, shade_n: Option<NormalMapTexture>) -> Point { todo!() }

    fn to_properties(&self, name: &String) -> Properties { todo!() }
}
