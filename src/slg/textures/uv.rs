use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureMapping2D, TextureType};

pub struct UVTexture {
    mapping: TextureMapping2D,
}

impl UVTexture {
    pub fn new(mapping: TextureMapping2D) -> Self { Self { mapping } }

    pub fn get_texture_mapping(&self) -> &TextureMapping2D { &self.mapping }
}

impl Texture for UVTexture {
    fn get_type(&self) -> TextureType { TextureType::UvTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { Spectrum::from((0.5, 0.5, 0.0)).y() }

    fn filter(&self) -> f32 { Spectrum::from((0.5, 0.5, 0.0)).filter() }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
