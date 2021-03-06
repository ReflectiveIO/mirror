use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureType};

pub struct BlenderNoiseTexture {
    noise_depth: i32,
    bright: f32,
    contrast: f32,
}

impl BlenderNoiseTexture {
    pub fn new(noise_depth: i32, bright: f32, contrast: f32) -> Self {
        Self {
            noise_depth,
            bright,
            contrast,
        }
    }

    pub fn get_noise_depth(&self) -> i32 { self.noise_depth }

    pub fn get_bright(&self) -> f32 { self.bright }

    pub fn get_contrast(&self) -> f32 { self.contrast }
}

impl Texture for BlenderNoiseTexture {
    fn get_type(&self) -> TextureType { TextureType::BlenderNoise }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 0.5 }

    fn filter(&self) -> f32 { 0.5 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
