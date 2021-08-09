use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::fresnel::texture::FresnelTexture;
use crate::slg::textures::{Texture, TextureType};

pub struct FresnelConstTexture {
    n: Spectrum,
    k: Spectrum,
}

impl FresnelConstTexture {
    pub fn new(n: Spectrum, k: Spectrum) -> Self { Self { n, k } }

    pub fn get_n(&self) -> &Spectrum { &self.n }

    pub fn get_k(&self) -> &Spectrum { &self.k }
}

impl Texture for FresnelConstTexture {
    fn get_type(&self) -> TextureType { TextureType::FresnelConstTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}

impl FresnelTexture for FresnelConstTexture {
    fn evaluate(&self, point: &HitPoint, cosi: f32) -> Spectrum { todo!() }
}
