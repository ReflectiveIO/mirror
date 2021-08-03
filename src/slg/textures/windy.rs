use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureMapping3D, TextureType};

pub struct WindyTexture {
    mapping: TextureMapping3D,
}

impl WindyTexture {
    pub fn new(mapping: TextureMapping3D) -> Self { Self { mapping } }

    pub fn get_texture_mapping(&self) -> &TextureMapping3D { &self.mapping }
}

impl Texture for WindyTexture {
    fn get_type(&self) -> TextureType { TextureType::Windy }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 0.5 }

    fn filter(&self) -> f32 { 0.5 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
