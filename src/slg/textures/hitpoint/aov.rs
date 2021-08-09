use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureType};

/// HitPointVertexAOV Texture
pub struct HitPointVertexAOVTexture {
    data_index: u32,
}

impl HitPointVertexAOVTexture {
    pub fn new(data_index: u32) -> Self { Self { data_index } }

    pub fn get_data_index(&self) -> u32 { self.data_index }
}

impl Texture for HitPointVertexAOVTexture {
    fn get_type(&self) -> TextureType { TextureType::HitPointVertexAov }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 1.0 }

    fn filter(&self) -> f32 { 1.0 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}

/// HitPointTriangleAOV Texture
pub struct HitPointTriangleAOVTexture {
    data_index: u32,
}

impl HitPointTriangleAOVTexture {
    pub fn new(data_index: u32) -> Self { Self { data_index } }

    pub fn get_data_index(&self) -> u32 { self.data_index }
}

impl Texture for HitPointTriangleAOVTexture {
    fn get_type(&self) -> TextureType { TextureType::HitPointTriangleAov }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 1.0 }

    fn filter(&self) -> f32 { 1.0 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
