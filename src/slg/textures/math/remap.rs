use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureType};

pub struct RemapTexture {
    value_tex: Box<dyn Texture>,
    source_min_tex: Box<dyn Texture>,
    source_max_tex: Box<dyn Texture>,
    target_min_tex: Box<dyn Texture>,
    target_max_tex: Box<dyn Texture>,
}

impl RemapTexture {
    pub fn new(
        value_tex: Box<dyn Texture>,
        source_min_tex: Box<dyn Texture>,
        source_max_tex: Box<dyn Texture>,
        target_min_tex: Box<dyn Texture>,
        target_max_tex: Box<dyn Texture>,
    ) -> Self {
        Self {
            value_tex,
            source_min_tex,
            source_max_tex,
            target_min_tex,
            target_max_tex,
        }
    }

    pub fn get_value_tex(&self) -> &Box<dyn Texture> { &self.value_tex }

    pub fn get_source_min_tex(&self) -> &Box<dyn Texture> { &self.source_min_tex }

    pub fn get_source_max_tex(&self) -> &Box<dyn Texture> { &self.source_max_tex }

    pub fn get_target_min_tex(&self) -> &Box<dyn Texture> { &self.target_min_tex }

    pub fn get_target_max_tex(&self) -> &Box<dyn Texture> { &self.target_max_tex }
}

impl Texture for RemapTexture {
    fn get_type(&self) -> TextureType { TextureType::RemapTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.value_tex.add_referenced_textures(v);
        self.source_min_tex.add_referenced_textures(v);
        self.source_max_tex.add_referenced_textures(v);
        self.target_min_tex.add_referenced_textures(v);
        self.target_max_tex.add_referenced_textures(v);
    }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        self.value_tex.add_referenced_image_maps(v);
        self.source_min_tex.add_referenced_image_maps(v);
        self.source_max_tex.add_referenced_image_maps(v);
        self.target_min_tex.add_referenced_image_maps(v);
        self.target_max_tex.add_referenced_image_maps(v);
    }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        if self.value_tex == old_tex {
            self.value_tex = new_tex.clone();
        }
        if self.source_min_tex == old_tex {
            self.source_min_tex = new_tex.clone();
        }
        if self.source_max_tex == old_tex {
            self.source_max_tex = new_tex.clone();
        }
        if self.target_min_tex == old_tex {
            self.target_min_tex = new_tex.clone();
        }
        if self.target_max_tex == old_tex {
            self.target_max_tex = new_tex.clone();
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
