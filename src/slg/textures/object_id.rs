use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureType};

pub struct ObjectIDTexture;

impl ObjectIDTexture {
    pub fn new() -> Self { Self }
}

impl Texture for ObjectIDTexture {
    fn get_type(&self) -> TextureType { TextureType::ObjectIdTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 1.0 }

    fn filter(&self) -> f32 { 1.0 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}

pub struct ObjectIDColorTexture;

impl ObjectIDColorTexture {
    pub fn new() -> Self { Self }
}

impl Texture for ObjectIDColorTexture {
    fn get_type(&self) -> TextureType { TextureType::ObjectIdColorTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 1.0 }

    fn filter(&self) -> f32 { 1.0 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}

pub struct ObjectIDNormalizedTexture;

impl ObjectIDNormalizedTexture {
    pub fn new() -> Self { Self }
}

impl Texture for ObjectIDNormalizedTexture {
    fn get_type(&self) -> TextureType { TextureType::ObjectIdNormalizedTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 1.0 }

    fn filter(&self) -> f32 { 1.0 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
