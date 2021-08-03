use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureType};

pub struct BlackBodyTexture {
    temperature: f32,
    normalize: bool,
    rgb: Spectrum,
}

impl BlackBodyTexture {
    pub fn new(temperature: f32, normalize: bool, rgb: Spectrum) -> Self {
        Self {
            temperature,
            normalize,
            rgb,
        }
    }

    pub fn get_rgb(&self) -> &Spectrum { &self.rgb }
}

impl Texture for BlackBodyTexture {
    fn get_type(&self) -> TextureType { TextureType::BlackBodyTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { self.rgb.y() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { self.rgb.clone() }

    fn y(&self) -> f32 { self.rgb.y() }

    fn filter(&self) -> f32 { self.rgb.filter() }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
