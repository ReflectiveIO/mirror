use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct WireFrameTexture {
    width: f32,
    border_texture: Box<dyn Texture>,
    inside_texture: Box<dyn Texture>,
}

impl WireFrameTexture {
    pub fn new(
        width: f32,
        border_texture: Box<dyn Texture>,
        inside_texture: Box<dyn Texture>,
    ) -> Self {
        Self {
            width,
            border_texture,
            inside_texture,
        }
    }

    pub fn get_width(&self) -> f32 { self.width }

    pub fn get_border_texture(&self) -> &Box<dyn Texture> { &self.border_texture }

    pub fn get_inside_texture(&self) -> &Box<dyn Texture> { &self.inside_texture }
}

impl Texture for WireFrameTexture {
    fn get_type(&self) -> TextureType { TextureType::WireFrameTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { (self.border_texture.y() + self.inside_texture.y()) * 0.5 }

    fn filter(&self) -> f32 {
        self(self.border_texture.filter() + self.inside_texture.filter()) * 0.5
    }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.border_texture.add_referenced_textures(v);
        self.inside_texture.add_referenced_textures(v)
    }

    fn add_referenced_image_maps(&mut self, v: &Vec<ImageMap>) {
        self.border_texture.add_referenced_image_maps(v);
        self.inside_texture.add_referenced_image_maps(v);
    }

    fn update_texture_references(&mut self, old_tex: &dyn Texture, new_tex: &dyn Texture) {
        if self.border_texture.as_ref() == old_tex {
            self.border_texture = Box::new(new_tex);
        }

        if self.inside_texture.as_ref() == old_tex {
            self.inside_texture = Box::new(new_tex)
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
