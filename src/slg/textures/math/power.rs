use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct PowerTexture {
    base: Box<dyn Texture>,
    exponent: Box<dyn Texture>,
}

impl PowerTexture {
    pub fn new(base: Box<dyn Texture>, exponent: Box<dyn Texture>) -> Self {
        Self { base, exponent }
    }

    pub fn get_base(&self) -> &Box<dyn Texture> { &self.base }

    pub fn get_exponent(&self) -> &Box<dyn Texture> { &self.exponent }
}

impl Texture for PowerTexture {
    fn get_type(&self) -> TextureType { TextureType::PowerTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.base.add_referenced_textures(v);
        self.exponent.add_referenced_textures(v);
    }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        self.base.add_referenced_image_maps(v);
        self.exponent.add_referenced_image_maps(v);
    }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        if self.base == old_tex {
            self.base = new_tex.clone();
        }
        if self.exponent == old_tex {
            self.exponent = new_tex.clone();
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
