use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct BilerpTexture {
    t00: Box<dyn Texture>,
    t01: Box<dyn Texture>,
    t10: Box<dyn Texture>,
    t11: Box<dyn Texture>,
}

impl BilerpTexture {
    pub fn new(
        t00: Box<dyn Texture>,
        t01: Box<dyn Texture>,
        t10: Box<dyn Texture>,
        t11: Box<dyn Texture>,
    ) -> Self {
        Self { t00, t01, t10, t11 }
    }

    pub fn get_texture00(&self) -> &Box<dyn Texture> { &self.t00 }

    pub fn get_texture01(&self) -> &Box<dyn Texture> { &self.t01 }

    pub fn get_texture10(&self) -> &Box<dyn Texture> { &self.t10 }

    pub fn get_texture11(&self) -> &Box<dyn Texture> { &self.t11 }
}

impl Texture for BilerpTexture {
    fn get_type(&self) -> TextureType { TextureType::BilerpTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) { todo!() }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) { todo!() }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        todo!()
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
