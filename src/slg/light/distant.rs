use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;

pub struct DistantLight {
    pub color: Spectrum,
    pub local_light_dir: Vector,
    pub theta: f32,
    abs_light_dir: Vector,
    x: Vector,
    y: Vector,
    sin2_theta_max: f32,
    cos_theta_max: f32,
}

impl DistantLight {
    pub fn new() -> Self {
        Self {
            color: Spectrum::new(),
            local_light_dir: Vector::new(0.0, 0.0, 1.0),
            theta: 0.0,

            abs_light_dir: Vector::new(0.0, 0.0, 0.0),
            x: Vector::new(0.0, 0.0, 0.0),
            y: Vector::new(0.0, 0.0, 0.0),
            sin2_theta_max: 0.0,
            cos_theta_max: 0.0,
        }
    }
}
