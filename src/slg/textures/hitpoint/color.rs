use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureType};

/// HitPointColor Texture
pub struct HitPointColorTexture {
    data_index: u32,
}

impl HitPointColorTexture {
    pub fn new(data_index: u32) -> Self { Self { data_index } }

    pub fn get_data_index(&self) -> u32 { self.data_index }
}

impl Texture for HitPointColorTexture {
    fn get_type(&self) -> TextureType { TextureType::HitPointColor }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 1.0 }

    fn filter(&self) -> f32 { 1.0 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}

/// HitPointAlpha Texture
pub struct HitPointAlphaTexture {
    data_index: u32,
}

impl HitPointAlphaTexture {
    pub fn new(data_index: u32) -> Self { Self { data_index } }

    pub fn get_data_index(&self) -> u32 { self.data_index }
}

impl Texture for HitPointAlphaTexture {
    fn get_type(&self) -> TextureType { TextureType::HitPointAlpha }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 1.0 }

    fn filter(&self) -> f32 { 1.0 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}

/// HitPointGrey Texture
pub struct HitPointGreyTexture {
    data_index: u32,
    channel: u32,
}

impl HitPointGreyTexture {
    pub fn new(data_index: u32, channel: u32) -> Self {
        Self {
            data_index,
            channel,
        }
    }

    pub fn get_data_index(&self) -> u32 { self.data_index }

    pub fn get_channel(&self) -> u32 { self.channel }
}

impl Texture for HitPointGreyTexture {
    fn get_type(&self) -> TextureType { TextureType::HitPointAlpha }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 1.0 }

    fn filter(&self) -> f32 { 1.0 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
