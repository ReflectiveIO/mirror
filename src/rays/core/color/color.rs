use crate::rays::utils::clamp;

/// Color Declarations
#[derive(PartialEq, Debug, Clone)]
pub struct Color(f32, f32, f32);

impl Color {
    pub fn new() -> Self {
        Color {
            0: 0.0,
            1: 0.0,
            2: 0.0,
        }
    }

    pub fn pow(&self, rhs: &Self) -> Self {
        Self {
            0: {
                if self.0 > 0.0 {
                    self.0.powf(rhs.0)
                } else {
                    0.0
                }
            },
            1: {
                if self.1 > 0.0 {
                    self.1.powf(rhs.1)
                } else {
                    0.0
                }
            },
            2: {
                if self.2 > 0.0 {
                    self.2.powf(rhs.2)
                } else {
                    0.0
                }
            },
        }
    }

    pub fn clamp(&self, low: f32, high: f32) -> Self {
        Self {
            0: clamp(self.0, low, high),
            1: clamp(self.1, low, high),
            2: clamp(self.2, low, high),
        }
    }

    pub fn is_nan(&self) -> bool { self.0.is_nan() || self.1.is_nan() || self.2.is_nan() }

    pub fn is_infinite(&self) -> bool {
        self.0.is_infinite() || self.1.is_infinite() || self.2.is_infinite()
    }

    pub fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative() || self.1.is_sign_negative() || self.2.is_sign_negative()
    }

    pub fn is_valid(&self) -> bool {
        !self.is_nan() && !self.is_infinite() && !self.is_sign_negative()
    }
}

impl From<f32> for Color {
    fn from(f: f32) -> Self { Self { 0: f, 1: f, 2: f } }
}

#[derive(Clone, PartialEq)]
pub struct RGBColor {
    color: Color,
}

impl RGBColor {
    pub fn new() -> Self {
        Self {
            color: Color::new(),
        }
    }

    pub fn y(&self) -> f32 {
        0.212671 * self.color.0 + 0.715160 * self.color.1 + 0.072169 * self.color.2
    }

    pub fn filter(&self) -> f32 { (self.color.0 + self.color.1 + self.color.2) * (1.0 / 3.0) }

    /// Required by OpenSubdivide interface
    pub fn clear(&mut self) {
        self.color.0 = 0.0;
        self.color.1 = 0.0;
        self.color.2 = 0.0;
    }

    pub fn add_with_weight(&mut self, src: &RGBColor, weight: f32) {
        self.color.0 += weight * src.color.0;
        self.color.1 += weight * src.color.1;
        self.color.2 += weight * src.color.2;
    }
}

impl From<f32> for RGBColor {
    fn from(f: f32) -> Self {
        Self {
            color: Color::from(f),
        }
    }
}

impl From<(f32, f32, f32)> for RGBColor {
    fn from(v: (f32, f32, f32)) -> Self {
        Self {
            color: Color {
                0: v.0,
                1: v.1,
                2: v.2,
            },
        }
    }
}

impl From<&Color> for RGBColor {
    fn from(color: &Color) -> Self {
        Self {
            color: Color {
                0: color.0,
                1: color.1,
                2: color.2,
            },
        }
    }
}

impl Default for RGBColor {
    fn default() -> Self { Self::new() }
}

pub type Spectrum = RGBColor;

#[test]
fn test_color_pow() {
    let c1 = Color::new();
    assert_eq!(&c1.pow(&Color::from(5.0)), &Color::new());
}
