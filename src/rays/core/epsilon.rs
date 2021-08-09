use crate::rays::geometry::Point;
use crate::rays::utils::clamp;

pub const DEFAULT_EPSILON_MIN: f32 = 1e-5f32;
pub const DEFAULT_EPSILON_MAX: f32 = 1e-1f32;
pub const DEFAULT_EPSILON_STATIC: f32 = 1e-5f32;

// An epsilon that can be used as threshold for cos(theta). For instance:
// if (dot(N, LightDir) < DEFAULT_COS_EPSILON_STATIC) return Spectrum();
pub const DEFAULT_COS_EPSILON_STATIC: f32 = 1e-4f32;

// This is about 1e-5f for values near 1.f
pub const DEFAULT_EPSILON_DISTANCE_FROM_VALUE: u8 = 0x80;

pub struct MachineEpsilon {
    min: f32,
    max: f32,
}

union MachineFloat {
    f: f32,
    i: u8,
}

impl MachineEpsilon {
    pub fn set_min(&mut self, min: f32) { self.min = min }

    pub fn get_min(&self) -> f32 { self.min }

    pub fn set_max(&mut self, max: f32) { self.max = max }

    pub fn get_max(&self) -> f32 { self.max }

    pub fn next(&mut self, value: f32) -> f32 {
        let mut mf = MachineFloat { f: value };
        mf.i += DEFAULT_EPSILON_DISTANCE_FROM_VALUE;

        return mf.f;
    }

    pub fn previous(&mut self, value: f32) -> f32 {
        let mut mf = MachineFloat { f: value };
        mf.i -= DEFAULT_EPSILON_DISTANCE_FROM_VALUE;

        return mf.f;
    }
}

impl Default for MachineEpsilon {
    fn default() -> Self {
        Self {
            min: DEFAULT_EPSILON_MIN,
            max: DEFAULT_EPSILON_MAX,
        }
    }
}

const MACHINE_EPSILON: MachineEpsilon = MachineEpsilon::default();

pub trait Epsilon<T> {
    fn epsilon(value: &T) -> f32;
}

impl Epsilon<f32> for MachineEpsilon {
    fn epsilon(value: &f32) -> f32 {
        clamp(
            (MACHINE_EPSILON.next(*value) - value).abs(),
            MACHINE_EPSILON.get_min(),
            MACHINE_EPSILON.get_max(),
        )
    }
}

impl Epsilon<Point> for MachineEpsilon {
    fn epsilon(value: &Point) -> f32 {
        f32::max(
            MachineEpsilon::epsilon(&value.x),
            f32::max(
                MachineEpsilon::epsilon(&value.y),
                MachineEpsilon::epsilon(&value.z),
            ),
        )
    }
}
