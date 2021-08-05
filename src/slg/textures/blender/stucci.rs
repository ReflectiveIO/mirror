use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureMapping3D, TextureType};

pub struct BlenderStucciTexture {
    mapping: TextureMapping3D,
    stucci_type: BlenderStucciType,
    noise_basis: BlenderNoiseBasis,
    noise_size: f32,
    turbulence: f32,
    hard: bool,
    bright: f32,
    contrast: f32,
}

impl BlenderStucciTexture {
    pub fn new(
        mapping: TextureMapping3D,
        stucci_type: BlenderStucciType,
        noise_basis: BlenderNoiseBasis,
        noise_size: f32,
        turbulence: f32,
        hard: bool,
        bright: f32,
        contrast: f32,
    ) -> Self {
        Self {
            mapping,
            stucci_type,
            noise_basis,
            noise_size,
            turbulence,
            hard,
            bright,
            contrast,
        }
    }

    pub fn get_texture_mapping(&self) -> &TextureMapping3D { &self.mapping }

    pub fn get_stucci_type(&self) -> &BlenderStucciType { &self.stucci_type }

    pub fn get_noise_basis(&self) -> &BlenderNoiseBasis { &self.noise_basis }

    pub fn get_noise_size(&self) -> f32 { self.noise_size }

    pub fn get_turbulence(&self) -> f32 { self.turbulence }

    pub fn get_noise_type(&self) -> bool { self.hard }

    pub fn get_bright(&self) -> f32 { self.bright }

    pub fn get_contrast(&self) -> f32 { self.contrast }
}

impl Texture for BlenderStucciTexture {
    fn get_type(&self) -> TextureType { TextureType::BlenderStucci }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 0.5 }

    fn filter(&self) -> f32 { 0.5 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
