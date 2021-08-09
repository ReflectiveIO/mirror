use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureType};

pub struct ShadingNormalTexture;

impl Texture for ShadingNormalTexture {
    fn get_type(&self) -> TextureType { TextureType::ShadingNormalTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    // The following methods don't make very much scene in this case.
    // I have no information about the color.
    fn y(&self) -> f32 { 1.0 }

    fn filter(&self) -> f32 { 1.0 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
