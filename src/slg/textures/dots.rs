use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureMapping2D, TextureType};

pub struct DotsTexture {
    mapping: Box<dyn TextureMapping2D>,
    inside_texture: Box<dyn Texture>,
    outside_texture: Box<dyn Texture>,
}

impl DotsTexture {
    pub fn new(
        mapping: Box<dyn TextureMapping2D>,
        inside_texture: Box<dyn Texture>,
        outside_texture: Box<dyn Texture>,
    ) -> Self {
        Self {
            mapping,
            inside_texture,
            outside_texture,
        }
    }

    pub fn get_texture_mapping(&self) -> &Box<dyn TextureMapping2D> { &self.mapping }

    pub fn get_inside_texture(&self) -> &Box<dyn Texture> { &self.inside_texture }

    pub fn get_outside_texture(&self) -> &Box<dyn Texture> { &self.outside_texture }
}

impl Texture for DotsTexture {
    fn get_type(&self) -> TextureType { TextureType::Dots }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { (self.inside_texture.y() + self.outside_texture.y()) * 0.5 }

    fn filter(&self) -> f32 { (self.inside_texture.filter() + self.outside_texture.filter()) * 0.5 }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.inside_texture.add_referenced_textures(v);
        self.outside_texture.add_referenced_textures(v);
    }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        self.inside_texture.add_referenced_image_maps(v);
        self.outside_texture.add_referenced_image_maps(v);
    }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        if self.inside_texture == old_tex {
            self.inside_texture = new_tex.clone();
        }

        if self.outside_texture == old_tex {
            self.outside_texture = new_tex.clone();
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
