use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct RandomTexture {
    tex: Box<dyn Texture>,
    seed_offset: u32,
}

impl RandomTexture {
    pub fn new(tex: Box<dyn Texture>, seed_offset: u32) -> Self { Self { tex, seed_offset } }

    pub fn get_texture(&self) -> &Box<dyn Texture> { &self.tex }
}

impl Texture for RandomTexture {
    fn get_type(&self) -> TextureType { TextureType::RandomTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { Spectrum::from(0.5).y() }

    fn filter(&self) -> f32 { 0.5 }

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
            self.tex = new_tex.clone();
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
