use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct SplitFloat3Texture {
    tex: Box<dyn Texture>,
    channel: u32,
}

impl SplitFloat3Texture {
    pub fn new(tex: Box<dyn Texture>, channel: u32) -> Self { Self { tex, channel } }

    pub fn get_texture(&self) -> &Box<dyn Texture> { &self.tex }

    pub fn get_channel(&self) -> u32 { self.channel }
}

impl Texture for SplitFloat3Texture {
    fn get_type(&self) -> TextureType { TextureType::SplitFloat3 }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 1.0 }

    fn filter(&self) -> f32 { 1.0 }

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
