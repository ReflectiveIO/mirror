use std::f32::consts::PI;

use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::utils::{clamp, Distribution2D};

#[inline]
pub fn spherical_theta(v: &Vector) -> f32 { clamp(v.z, -1.0, 1.0).acos() }

#[inline]
pub fn spherical_phi(v: &Vector) -> f32 {
    let p: f32 = v.y.atan2(v.x);

    if p < 0.0 {
        p + 2.0 * PI
    } else {
        p
    }
}

/// A simple interface for functions on a sphere.
pub trait SphericalFunction {
    fn eval(&self, w: &Vector) -> Spectrum { self.evaluate(spherical_phi(w), spherical_theta(w)) }

    fn evaluate(&self, phi: f32, theta: f32) -> Spectrum;
}

pub struct NopSphericalFunction;

impl SphericalFunction for NopSphericalFunction {
    fn evaluate(&self, phi: f32, theta: f32) -> Spectrum { Spectrum::from(1.0) }
}

pub struct SampleableSphericalFunction {
    func: Box<dyn SphericalFunction>,
    uv_dist2d: Distribution2D,
    average: f32,
}

impl SampleableSphericalFunction {
    pub fn new(func: Box<dyn SphericalFunction>, x: u32, y: u32) -> Self {
        Self {
            func,
            uv_dist2d: Default::default(),
            average: 0.0,
        }
    }

    /// Samples this spherical function
    ///
    /// u1 - The first random value.
    /// u2 - The second random value.
    /// w - The address to store the sampled direction in.
    /// pdf - The address to store the pdf (w.r.t. solid angle) of the sample
    /// direction in.
    pub fn sample(&self, u1: f32, u2: f32, w: &Vector, pdf: f32) -> Spectrum { Spectrum::new() }

    /// Computes the pdf for sampling the given direction.
    pub fn pdf(&self, w: &Vector) -> f32 { 0.0 }

    /// Returns the average function value over the entire sphere.
    pub fn average(&self) -> f32 { 0.0 }

    pub fn get_func(&self) -> &Box<dyn SphericalFunction> { &self.func }

    pub fn get_distribution2d(&self) -> &Distribution2D { &self.uv_dist2d }
}

impl SphericalFunction for SampleableSphericalFunction {
    fn evaluate(&self, phi: f32, theta: f32) -> Spectrum { todo!() }
}

impl Default for SampleableSphericalFunction {
    fn default() -> Self {
        Self {
            func: Box::new(NopSphericalFunction),
            uv_dist2d: Default::default(),
            average: 0.0,
        }
    }
}
