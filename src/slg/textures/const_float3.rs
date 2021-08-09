use crate::rays::color::Spectrum;
use crate::rays::geometry::Normal;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::texture::TextureType;
use crate::slg::textures::Texture;

pub struct ConstFloat3Texture {
    color: Spectrum,
}

impl ConstFloat3Texture {
    pub fn new(c: &Spectrum) -> Self { Self { color: c.clone() } }

    pub fn get_color(&self) -> &Spectrum { &self.color }
}

impl Texture for ConstFloat3Texture {
    fn get_type(&self) -> TextureType { TextureType::ConstFloat3 }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { self.color.y() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { self.color.clone() }

    fn y(&self) -> f32 { self.color.y() }

    fn filter(&self) -> f32 { self.color.filter() }

    fn bump(&self, hp: &HitPoint, sample_distance: f32) -> Normal { todo!() }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
