use crate::rays;
use crate::rays::color::Spectrum;
use crate::slg::bsdf::hitpoint::HitPoint;

/// Fresnel texture interface
pub trait FresnelTextureTrait {
    fn evaluate(&self, point: &HitPoint, cosi: f32) -> Box<Spectrum>;
}

pub struct FresnelTexture;

impl FresnelTexture {
    // static
    pub fn approx_nf32(fr: f32) -> f32 {
        let sqrt_reflectance: f32 = (rays::clamp(fr, 0.0_f32, 0.999_f32)).sqrt();
        (1.0_f32 + sqrt_reflectance) / (1.0_f32 - sqrt_reflectance)
    }

    pub fn approx_n(fr: &Spectrum) -> Box<Spectrum> {
        let sqrt_reflectance = fr.clamp(0.0_f32, 0.999_f32).sqrt();
        (Spectrum::new(1.0_f32) + sqrt_reflectance) / (Spectrum::new(1.0_f32) - sqrt_reflectance)
    }

    pub fn approx_kf32(fr: f32) -> f32 {}
    pub fn approx_k(fr: &Spectrum) -> Box<Spectrum> {}

    pub fn schlick_evaluate(ks: &Spectrum, cosi: f32) -> Box<Spectrum> {}
    pub fn general_evaluate(eta: &Spectrum, k: &Spectrum, cosi: f32) {}
    pub fn cauchy_evaluate(eta: f32, cosi: f32) -> f32 {}

    // private static
    fn fr_full(cosi: f32, cost: &Spectrum, eta: &Spectrum, k: &Spectrum) -> Box<Spectrum> {}
    fn fr_diel2(cosi: f32, cost: f32, eta: f32) -> f32 {}
}
