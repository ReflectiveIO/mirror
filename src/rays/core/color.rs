use crate::rays;

/// Color Declarations
#[derive(PartialEq, Debug)]
pub struct Color(f32, f32, f32);

impl Color {
    pub fn new(f: f32) -> Self {
        Color { 0: f, 1: f, 2: f }
    }

    pub fn pow(&self, rhs: &Self) -> Self {
        Self {
            0: { if self.0 > 0.0 { self.0.powf(rhs.0) } else { 0.0 } },
            1: { if self.1 > 0.0 { self.1.powf(rhs.1) } else { 0.0 } },
            2: { if self.2 > 0.0 { self.2.powf(rhs.2) } else { 0.0 } },
        }
    }

    pub fn clamp(&self, low: f32, high: f32) -> Self {
        Self {
            0: rays::clamp(self.0, low, high),
            1: rays::clamp(self.1, low, high),
            2: rays::clamp(self.2, low, high),
        }
    }
}

pub trait ColorPowTrait<T> {}

pub trait ColorTrait {}

pub trait RGBColor: ColorTrait {}

pub type Spectrum = dyn RGBColor;

#[test]
fn test_color_pow() {
    let c1 = Color::new(0.0);
    assert_eq!(&c1.pow(&Color::new(5.0)), &Color::new(0.0));
}
