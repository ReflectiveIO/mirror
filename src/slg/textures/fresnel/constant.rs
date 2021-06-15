use crate::rays::core::color::{RGBColor, Spectrum};
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::textures::fresnel::texture::FresnelTextureTrait;

pub struct FresnelConstTexture;

pub trait FresnelConstTextureTrait {
    fn get_type(&self);
}

impl FresnelTextureTrait for FresnelConstTexture {
    fn evaluate(&self, point: &HitPoint, cosi: f32) -> Box<Spectrum> {
        unimplemented!()
    }
}

impl FresnelConstTextureTrait for FresnelConstTexture {
    fn get_type(&self) {
        unimplemented!()
    }
}

impl FresnelConstTexture {
    pub fn new() -> Self {
        FresnelConstTexture
    }

    pub fn filter(&self) -> f32 {
        0.0f32
    }
}

#[test]
fn test_fresnel_const_texture_filter() {
    let texture: FresnelConstTexture = FresnelConstTexture::new();
    assert_eq!(texture.filter(), 0.0f32);
}
