use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct DistortTexture {
    texture: Box<dyn Texture>,
    offset: Box<dyn Texture>,
    strength: f32,
}

impl DistortTexture {
    pub fn new(texture: Box<dyn Texture>, offset: Box<dyn Texture>, strength: f32) -> Self {
        Self {
            texture,
            offset,
            strength,
        }
    }

    pub fn get_texture(&self) -> &Box<dyn Texture> { &self.texture }

    pub fn get_offset(&self) -> &Box<dyn Texture> { &self.offset }

    pub fn get_strength(&self) -> f32 { self.strength }
}

impl Texture for DistortTexture {
    fn get_type(&self) -> TextureType { TextureType::DistortTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { self.texture.y() }

    fn filter(&self) -> f32 { self.texture.filter() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.texture.add_referenced_textures(v);
        self.offset.add_referenced_textures(v);
    }

    fn add_referenced_image_maps(&mut self, v: &Vec<ImageMap>) {
        self.texture.add_referenced_image_maps(v);
        self.offset.add_referenced_image_maps(v);
    }

    fn update_texture_references(&mut self, old_tex: &dyn Texture, new_tex: &dyn Texture) {
        if self.texture.as_ref() == old_tex {
            self.texture = Box::new(new_tex);
        }

        if self.offset.as_ref() == old_tex {
            self.offset = Box::new(new_tex);
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
