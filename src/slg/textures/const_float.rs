use crate::rays::color::Spectrum;
use crate::rays::geometry::Normal;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::texture::TextureType;
use crate::slg::textures::Texture;

pub struct ConstFloatTexture {
    value: f32,
}

impl ConstFloatTexture {
    pub fn new(v: f32) -> Self { Self { value: v } }

    pub fn get_value(&self) -> f32 { self.value }
}

impl Texture for ConstFloatTexture {
    fn get_type(&self) -> TextureType { TextureType::ConstFloat }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { self.value }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { Spectrum::from(self.value) }

    fn y(&self) -> f32 { self.value }

    fn filter(&self) -> f32 { self.value }

    fn bump(&self, hp: &HitPoint, sample_distance: f32) -> Normal { todo!() }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
