use crate::rays::color::Spectrum;
use crate::rays::geometry::Normal;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct BevelTexture {
    texture: Option<Box<dyn Texture>>,
    radius: f32,
}

impl BevelTexture {
    pub fn new(texture: Option<Box<dyn Texture>>, radius: f32) -> Self { Self { texture, radius } }

    pub fn get_texture(&self) -> &Option<Box<dyn Texture>> { &self.texture }

    pub fn get_radius(&self) -> f32 { self.radius }
}

impl Texture for BevelTexture {
    fn get_type(&self) -> TextureType { TextureType::BevelTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { 0.0 }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { Spectrum::new() }

    fn y(&self) -> f32 { 0.0 }

    fn filter(&self) -> f32 { 0.0 }

    fn bump(&self, hp: &HitPoint, sample_distance: f32) -> Normal { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        if let Some(mut tex) = &self.texture {
            tex.add_referenced_textures(v)
        }
    }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        if let Some(mut tex) = &self.texture {
            tex.add_referenced_image_maps(v);
        }
    }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        if self.texture.is_some() && self.texture.unwrap() == old_tex {
            self.texture = Some(new_tex.clone())
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
