use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{ProgressionType, Texture, TextureMapping3D, TextureType};

pub struct BlenderBlendTexture {
    mapping: Box<dyn TextureMapping3D>,
    progression_type: ProgressionType,
    direction: bool,
    bright: f32,
    contrast: f32,
}

impl BlenderBlendTexture {
    pub fn new(
        mapping: Box<dyn TextureMapping3D>,
        progression_type: ProgressionType,
        direction: bool,
        bright: f32,
        contrast: f32,
    ) -> Self {
        Self {
            mapping,
            progression_type,
            direction,
            bright,
            contrast,
        }
    }

    pub fn get_texture_mapping(&self) -> &Box<dyn TextureMapping3D> { &self.mapping }

    pub fn get_progression_type(&self) -> &ProgressionType { &self.progression_type }

    pub fn get_direction(&self) -> bool { self.direction }

    pub fn get_bright(&self) -> f32 { self.bright }

    pub fn get_contrast(&self) -> f32 { self.contrast }
}

impl Texture for BlenderBlendTexture {
    fn get_type(&self) -> TextureType { TextureType::BlenderBlend }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 0.5 }

    fn filter(&self) -> f32 { 0.5 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
