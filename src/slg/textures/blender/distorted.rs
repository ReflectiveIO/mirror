use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureMapping3D, TextureType};

pub struct BlenderDistortedNoiseTexture {
    mapping: TextureMapping3D,
    noise_distortion: BlenderNoiseBasis,
    noise_basis: BlenderNoiseBasis,
    distortion: i32,
    noise_size: f32,
    bright: f32,
    contrast: f32,
}

impl BlenderDistortedNoiseTexture {
    pub fn new(
        mapping: TextureMapping3D,
        noise_distortion: BlenderNoiseBasis,
        noise_basis: BlenderNoiseBasis,
        distortion: i32,
        noise_size: f32,
        bright: f32,
        contrast: f32,
    ) -> Self {
        Self {
            mapping,
            noise_distortion,
            noise_basis,
            distortion,
            noise_size,
            bright,
            contrast,
        }
    }

    pub fn get_texture_mapping(&self) -> &TextureMapping3D { &self.mapping }

    pub fn get_noise_distortion(&self) -> &BlenderNoiseBasis { &self.noise_distortion }

    pub fn get_noise_basis(&self) -> &BlenderNoiseBasis { &self.noise_basis }

    pub fn get_distortion(&self) -> i32 { self.distortion }

    pub fn get_noise_size(&self) -> f32 { self.noise_size }

    pub fn get_bright(&self) -> f32 { self.bright }

    pub fn get_contrast(&self) -> f32 { self.contrast }
}

impl Texture for BlenderDistortedNoiseTexture {
    fn get_type(&self) -> TextureType { TextureType::BlenderDistortedNoise }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 0.5 }

    fn filter(&self) -> f32 { 0.5 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
