use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureMapping2D, TextureType};

pub struct BombingTexture {
    mapping: Box<dyn TextureMapping2D>,
    background_texture: Box<dyn Texture>,
    bullet_texture: Box<dyn Texture>,
    bullet_mask_texture: Box<dyn Texture>,

    random_scale_factor: f32,
    use_random_rotation: bool,
    multi_bullet_count: u32,
}

impl BombingTexture {
    pub fn new(
        mapping: Box<dyn TextureMapping2D>,
        background_texture: Box<dyn Texture>,
        bullet_texture: Box<dyn Texture>,
        bullet_mask_texture: Box<dyn Texture>,
        random_scale_factor: f32,
        use_random_rotation: bool,
        multi_bullet_count: u32,
    ) -> Self {
        Self {
            mapping,
            background_texture,
            bullet_texture,
            bullet_mask_texture,
            random_scale_factor,
            use_random_rotation,
            multi_bullet_count,
        }
    }

    pub fn get_texture_mapping(&self) -> &Box<dyn TextureMapping2D> { &self.mapping }

    pub fn get_background_texture(&self) -> &Box<dyn Texture> { &self.background_texture }

    pub fn get_bullet_texture(&self) -> &Box<dyn Texture> { &self.bullet_texture }

    pub fn get_bullet_mask_texture(&self) -> &Box<dyn Texture> { &self.bullet_mask_texture }

    pub fn get_random_scale_factor(&self) -> f32 { self.random_scale_factor }

    pub fn get_use_random_rotation(&self) -> bool { self.use_random_rotation }

    pub fn get_multi_bullet_count(&self) -> u32 { self.multi_bullet_count }
}

impl Texture for BombingTexture {
    fn get_type(&self) -> TextureType { TextureType::BombingTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) { todo!() }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) { todo!() }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        todo!()
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
