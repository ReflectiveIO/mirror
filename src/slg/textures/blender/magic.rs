use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureMapping3D, TextureType};

pub struct BlenderMagicTexture {
    mapping: Box<dyn TextureMapping3D>,
    noise_depth: i32,
    turbulence: f32,
    bright: f32,
    contrast: f32,
}

impl BlenderMagicTexture {
    pub fn new(
        mapping: Box<dyn TextureMapping3D>,
        noise_depth: i32,
        turbulence: f32,
        bright: f32,
        contrast: f32,
    ) -> Self {
        Self {
            mapping,
            noise_depth,
            turbulence,
            bright,
            contrast,
        }
    }

    pub fn get_texture_mapping(&self) -> &Box<dyn TextureMapping3D> { &self.mapping }

    pub fn get_noise_depth(&self) -> i32 { self.noise_depth }

    pub fn get_turbulence(&self) -> f32 { self.turbulence }

    pub fn get_bright(&self) -> f32 { self.bright }

    pub fn get_contrast(&self) -> f32 { self.contrast }
}

impl Texture for BlenderMagicTexture {
    fn get_type(&self) -> TextureType { TextureType::BlenderMagic }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
