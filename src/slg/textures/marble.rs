use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureMapping3D, TextureType};

pub struct MarbleTexture {
    mapping: Box<dyn TextureMapping3D>,
    octaves: i32,
    omega: f32,
    scale: f32,
    variation: f32,
}

impl MarbleTexture {
    pub fn new(
        mapping: Box<dyn TextureMapping3D>,
        octaves: i32,
        omega: f32,
        scale: f32,
        variation: f32,
    ) -> Self {
        Self {
            mapping,
            octaves,
            omega,
            scale,
            variation,
        }
    }

    pub fn get_texture_mapping(&self) -> &Box<dyn TextureMapping3D> { &self.mapping }

    pub fn get_octaves(&self) -> i32 { self.octaves }

    pub fn get_omega(&self) -> f32 { self.omega }

    pub fn get_scale(&self) -> f32 { self.scale }

    pub fn get_variation(&self) -> f32 { self.variation }
}

impl Texture for MarbleTexture {
    fn get_type(&self) -> TextureType { TextureType::Marble }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
