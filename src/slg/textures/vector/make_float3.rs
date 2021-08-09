use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct MakeFloat3Texture {
    tex1: Box<dyn Texture>,
    tex2: Box<dyn Texture>,
    tex3: Box<dyn Texture>,
}

impl MakeFloat3Texture {
    pub fn new(tex1: Box<dyn Texture>, tex2: Box<dyn Texture>, tex3: Box<dyn Texture>) -> Self {
        Self { tex1, tex2, tex3 }
    }

    pub fn get_texture1(&self) -> &Box<dyn Texture> { &self.tex1 }

    pub fn get_texture2(&self) -> &Box<dyn Texture> { &self.tex2 }

    pub fn get_texture3(&self) -> &Box<dyn Texture> { &self.tex3 }
}

impl Texture for MakeFloat3Texture {
    fn get_type(&self) -> TextureType { TextureType::MakeFloat3 }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 1.0 }

    fn filter(&self) -> f32 { 1.0 }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.tex1.add_referenced_textures(v);
        self.tex2.add_referenced_textures(v);
        self.tex3.add_referenced_textures(v);
    }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        self.tex1.add_referenced_image_maps(v);
        self.tex2.add_referenced_image_maps(v);
        self.tex3.add_referenced_image_maps(v);
    }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        if self.tex1 == old_tex {
            self.tex1 = new_tex.clone();
        }
        if self.tex2 == old_tex {
            self.tex2 = new_tex.clone();
        }
        if self.tex3 == old_tex {
            self.tex3 = new_tex.clone();
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
