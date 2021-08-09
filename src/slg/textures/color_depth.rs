use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct ColorDepthTexture {
    d: f32,
    kt: Box<dyn Texture>,
}

impl ColorDepthTexture {
    pub fn new(depth: f32, t: Box<dyn Texture>) -> Self {
        Self {
            d: f32::max(1e-3, depth),
            kt: t,
        }
    }

    pub fn get_d(&self) -> f32 { self.d }

    pub fn get_kt(&self) -> &Box<dyn Texture> { &self.kt }
}

impl Texture for ColorDepthTexture {
    fn get_type(&self) -> TextureType { TextureType::ColorDepthTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.kt.add_referenced_textures(v);
    }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        self.kt.add_referenced_image_maps(v);
    }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        if self.kt == old_tex {
            self.kt = new_tex.clone();
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
