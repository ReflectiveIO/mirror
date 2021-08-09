use crate::rays;
use crate::rays::color::Spectrum;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::textures::Texture;

/// Fresnel texture interface
pub trait FresnelTexture: Texture {
    fn evaluate(&self, point: &HitPoint, cosi: f32) -> Spectrum;
}
