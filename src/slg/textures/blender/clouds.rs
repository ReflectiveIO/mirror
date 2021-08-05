use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{BlenderNoiseBasis, Texture, TextureMapping3D, TextureType};

pub struct BlenderCloudsTexture {
    mapping: Box<dyn TextureMapping3D>,
    noise_basis: BlenderNoiseBasis,
    noise_depth: i32,
    noise_size: f32,
    hard: bool,
    bright: f32,
    contrast: f32,
}

impl BlenderCloudsTexture {
    pub fn new(
        mapping: Box<dyn TextureMapping3D>,
        noise_basis: BlenderNoiseBasis,
        noise_depth: i32,
        noise_size: f32,
        hard: bool,
        bright: f32,
        contrast: f32,
    ) -> Self {
        Self {
            mapping,
            noise_basis,
            noise_depth,
            noise_size,
            hard,
            bright,
            contrast,
        }
    }

    pub fn get_texture_mapping(&self) -> &Box<dyn TextureMapping3D> { &self.mapping }

    pub fn get_noise_basis(&self) -> &BlenderNoiseBasis { &self.noise_basis }

    pub fn get_noise_depth(&self) -> i32 { self.noise_depth }

    pub fn get_noise_size(&self) -> f32 { self.noise_size }

    pub fn get_noise_type(&self) -> bool { self.hard }

    pub fn get_bright(&self) -> f32 { self.bright }

    pub fn get_contrast(&self) -> f32 { self.contrast }
}

impl Texture for BlenderCloudsTexture {
    fn get_type(&self) -> TextureType { TextureType::BlenderClouds }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 0.5 }

    fn filter(&self) -> f32 { 0.5 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
