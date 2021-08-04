use crate::rays::color::Spectrum;
use crate::rays::geometry::Normal;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureMapping3D, TextureType};

pub struct TriplanarTexture {
    mapping: TextureMapping3D,
    text_x: Box<dyn Texture>,
    text_y: Box<dyn Texture>,
    text_z: Box<dyn Texture>,
    enable_uv_less_bump_map: bool,
}

impl TriplanarTexture {
    pub fn new(
        mapping: TextureMapping3D,
        text_x: Box<dyn Texture>,
        text_y: Box<dyn Texture>,
        text_z: Box<dyn Texture>,
        enable_uv_less_bump_map: bool,
    ) -> Self {
        Self {
            mapping,
            text_x,
            text_y,
            text_z,
            enable_uv_less_bump_map,
        }
    }

    pub fn get_texture_mapping(&self) -> &TextureMapping3D { &self.mapping }

    pub fn get_texture1(&self) -> &Box<dyn Texture> { &self.text_x }

    pub fn get_texture2(&self) -> &Box<dyn Texture> { &self.text_y }

    pub fn get_texture3(&self) -> &Box<dyn Texture> { &self.text_z }

    pub fn is_uv_less_bump_map(&self) -> bool { self.enable_uv_less_bump_map }
}

impl Texture for TriplanarTexture {
    fn get_type(&self) -> TextureType { TextureType::TriplanarTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { (self.text_x.y() + self.text_y.y() + self.text_z.y()) * (1.0 / 3.0) }

    fn filter(&self) -> f32 {
        (self.text_x.filter() + self.text_y.filter() + self.text_z.filter()) * (1.0 / 3.0)
    }

    fn bump(&self, hp: &HitPoint, sample_distance: f32) -> Normal { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.text_x.add_referenced_textures(v);
        self.text_y.add_referenced_textures(v);
        self.text_z.add_referenced_textures(v);
    }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        self.text_x.add_referenced_image_maps(v);
        self.text_y.add_referenced_image_maps(v);
        self.text_z.add_referenced_image_maps(v);
    }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        if self.text_x == old_tex {
            self.text_x = new_tex.clone();
        }

        if self.text_y == old_tex {
            self.text_y = new_tex.clone();
        }

        if self.text_z == old_tex {
            self.text_z = new_tex.clone()
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
