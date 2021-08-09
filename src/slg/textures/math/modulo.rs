use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct ModuloTexture {
    texture: Box<dyn Texture>,
    modulo: Box<dyn Texture>,
}

impl ModuloTexture {
    pub fn new(texture: Box<dyn Texture>, modulo: Box<dyn Texture>) -> Self {
        Self { texture, modulo }
    }

    pub fn get_texture(&self) -> &Box<dyn Texture> { &self.texture }

    pub fn get_modulo(&self) -> &Box<dyn Texture> { &self.modulo }
}

impl Texture for ModuloTexture {
    fn get_type(&self) -> TextureType { TextureType::ModuloTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.texture.add_referenced_textures(v);
        self.modulo.add_referenced_textures(v);
    }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        self.texture.add_referenced_image_maps(v);
        self.modulo.add_referenced_image_maps(v);
    }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        if self.texture == old_tex {
            self.texture = new_tex.clone();
        }
        if self.modulo == old_tex {
            self.modulo = new_tex.clone();
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
