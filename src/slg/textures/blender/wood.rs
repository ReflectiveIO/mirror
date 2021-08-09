use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{
    BlenderNoiseBase, BlenderNoiseBasis, BlenderWoodType, Texture, TextureMapping3D, TextureType,
};

pub struct BlenderWoodTexture {
    mapping: Box<dyn TextureMapping3D>,
    wood_type: BlenderWoodType,
    noise_basis: BlenderNoiseBasis,
    noise_basis2: BlenderNoiseBase,
    noise_size: f32,
    turbulence: f32,
    hard: bool,
    bright: f32,
    contrast: f32,
}

impl BlenderWoodTexture {
    pub fn new(
        mapping: Box<dyn TextureMapping3D>,
        wood_type: BlenderWoodType,
        noise_basis: BlenderNoiseBasis,
        noise_basis2: BlenderNoiseBase,
        noise_size: f32,
        turbulence: f32,
        hard: bool,
        bright: f32,
        contrast: f32,
    ) -> Self {
        Self {
            mapping,
            wood_type,
            noise_basis,
            noise_basis2,
            noise_size,
            turbulence,
            hard,
            bright,
            contrast,
        }
    }

    pub fn get_texture_mapping(&self) -> &Box<dyn TextureMapping3D> { &self.mapping }

    pub fn get_wood_type(&self) -> &BlenderWoodType { &self.wood_type }

    pub fn get_noise_basis(&self) -> &BlenderNoiseBasis { &self.noise_basis }

    pub fn get_noise_basis2(&self) -> &BlenderNoiseBase { &self.noise_basis2 }

    pub fn get_noise_size(&self) -> f32 { self.noise_size }

    pub fn get_turbulence(&self) -> f32 { self.turbulence }

    pub fn get_noise_type(&self) -> bool { self.hard }

    pub fn get_bright(&self) -> f32 { self.bright }

    pub fn get_contrast(&self) -> f32 { self.contrast }
}

impl Texture for BlenderWoodTexture {
    fn get_type(&self) -> TextureType { TextureType::BlenderWood }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 0.5 }

    fn filter(&self) -> f32 { 0.5 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
