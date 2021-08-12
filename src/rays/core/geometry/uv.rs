use std::ops::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UV {
    pub u: f32,
    pub v: f32,
}

impl UV {
    pub fn new(u: f32, v: f32) -> Self { Self { u, v } }

    pub fn is_nan(&self) -> bool { self.u.is_nan() || self.v.is_nan() }

    pub fn is_infinite(&self) -> bool { self.u.is_infinite() || self.v.is_infinite() }

    pub fn clear(&mut self) {
        self.u = 0.0;
        self.v = 0.0;
    }

    pub fn add_with_weight(&mut self, src: &UV, weight: f32) {
        self.u += weight * src.u;
        self.v += weight * src.v;
    }
}

/// The addition operator +.
impl Add for UV {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            u: self.u + rhs.u,
            v: self.v + rhs.v,
        }
    }
}

/// The addition assignment operator +=.
impl AddAssign for UV {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            u: self.u + rhs.u,
            v: self.v + rhs.v,
        };
    }
}

/// The subtraction operator -.
impl Sub for UV {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            u: self.u - rhs.u,
            v: self.v - rhs.v,
        }
    }
}

/// The subtraction assignment operator -=.
impl SubAssign for UV {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            u: self.u - rhs.u,
            v: self.v - rhs.v,
        };
    }
}

impl Mul<f32> for UV {
    type Output = UV;

    fn mul(self, rhs: f32) -> Self::Output { Self::new(self.u * rhs, self.v * rhs) }
}

impl MulAssign<f32> for UV {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            u: self.u * rhs,
            v: self.v * rhs,
        }
    }
}

/// The division operator /.
impl Div<f32> for UV {
    type Output = UV;

    fn div(self, rhs: f32) -> Self::Output {
        let inv = 1.0 / rhs;
        Self::new(self.u * inv, self.v * inv)
    }
}

impl DivAssign<f32> for UV {
    fn div_assign(&mut self, rhs: f32) {
        let inv = 1.0 / rhs;
        *self = Self {
            u: self.u * inv,
            v: self.v * inv,
        }
    }
}

impl Index<usize> for UV {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.u,
            1 => &self.v,
            _ => {
                panic!("Bad index range of UV")
            },
        }
    }
}

impl Mul<UV> for f32 {
    type Output = UV;

    #[inline]
    fn mul(self, rhs: UV) -> Self::Output { rhs * self }
}
