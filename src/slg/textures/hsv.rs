use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

/// Hue saturation value texture
pub struct HsvTexture {
    tex: Box<dyn Texture>,
    hue: Box<dyn Texture>,
    sat: Box<dyn Texture>,
    val: Box<dyn Texture>,
}

impl HsvTexture {
    pub fn new(
        tex: Box<dyn Texture>,
        hue: Box<dyn Texture>,
        sat: Box<dyn Texture>,
        val: Box<dyn Texture>,
    ) -> Self {
        Self { tex, hue, sat, val }
    }

    pub fn get_texture(&self) -> &Box<dyn Texture> { &self.tex }

    pub fn get_hue(&self) -> &Box<dyn Texture> { &self.hue }

    pub fn get_saturation(&self) -> &Box<dyn Texture> { &self.sat }

    pub fn get_value(&self) -> &Box<dyn Texture> { &self.val }
}

impl Texture for HsvTexture {
    fn get_type(&self) -> TextureType { TextureType::HsvTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.tex.add_referenced_textures(v);
        self.hue.add_referenced_textures(v);
        self.sat.add_referenced_textures(v);
        self.val.add_referenced_textures(v);
    }

    fn add_referenced_image_maps(&mut self, v: &Vec<ImageMap>) {
        self.tex.add_referenced_image_maps(v);
        self.hue.add_referenced_image_maps(v);
        self.sat.add_referenced_image_maps(v);
        self.val.add_referenced_image_maps(v);
    }

    fn update_texture_references(&mut self, old_tex: &dyn Texture, new_tex: &dyn Texture) {
        if self.tex.as_ref() == old_tex {
            self.tex = Box::new(new_tex);
        }
        if self.hue.as_ref() == old_tex {
            self.hue = Box::new(new_tex);
        }
        if self.sat.as_ref() == old_tex {
            self.sat = Box::new(new_tex);
        }
        if self.val.as_ref() == old_tex {
            self.val = Box::new(new_tex);
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
