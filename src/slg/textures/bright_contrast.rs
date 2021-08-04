use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct BrightContrastTexture {
    tex: Box<dyn Texture>,
    brightness_tex: Box<dyn Texture>,
    contrast_tex: Box<dyn Texture>,
}

impl BrightContrastTexture {
    pub fn new(
        tex: Box<dyn Texture>,
        brightness_tex: Box<dyn Texture>,
        contrast_tex: Box<dyn Texture>,
    ) -> Self {
        Self {
            tex,
            brightness_tex,
            contrast_tex,
        }
    }

    pub fn get_tex(&self) -> &Box<dyn Texture> { &self.tex }

    pub fn get_brightness_tex(&self) -> &Box<dyn Texture> { &self.brightness_tex }

    pub fn get_contrast_tex(&self) -> &Box<dyn Texture> { &self.contrast_tex }
}

impl Texture for BrightContrastTexture {
    fn get_type(&self) -> TextureType { TextureType::BrightContrastTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 {
        0.0 //@TODO
    }

    fn filter(&self) -> f32 {
        0.0 //@TODO
    }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.tex.add_referenced_textures(v);
        self.brightness_tex.add_referenced_textures(v);
        self.contrast_tex.add_referenced_textures(v);
    }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        self.tex.add_referenced_image_maps(v);
        self.brightness_tex.add_referenced_image_maps(v);
        self.contrast_tex.add_referenced_image_maps(v);
    }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        if self.tex == old_tex {
            self.tex = new_tex.clone();
        }
        if self.brightness_tex == old_tex {
            self.brightness_tex = new_tex.clone();
        }
        if self.contrast_tex == old_tex {
            self.contrast_tex = new_tex.clone();
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
