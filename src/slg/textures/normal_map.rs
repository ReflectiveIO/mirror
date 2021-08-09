use crate::rays::color::Spectrum;
use crate::rays::geometry::Normal;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct NormalMapTexture {
    tex: Box<dyn Texture>,
    scale: f32,
}

impl NormalMapTexture {
    pub fn new(tex: Box<dyn Texture>, scale: f32) -> Self { Self { tex, scale } }

    pub fn get_texture(&self) -> &Box<dyn Texture> { &self.tex }

    pub fn get_scale(&self) -> f32 { self.scale }
}

impl Texture for NormalMapTexture {
    fn get_type(&self) -> TextureType { TextureType::NormalMapTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { 0.0 }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { Spectrum::default() }

    fn y(&self) -> f32 { 0.0 }

    fn filter(&self) -> f32 { 0.0 }

    fn bump(&self, hp: &HitPoint, sample_distance: f32) -> Normal { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.tex.add_referenced_textures(v);
    }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        self.tex.add_referenced_image_maps(v);
    }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        if self.tex == old_tex {
            self.tex = new_tex.clone()
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
