use typed_builder::TypedBuilder;

use crate::rays::epsilon::{Epsilon, MachineEpsilon};
use crate::rays::geometry::{Point, Vector};

const RAY_FLAGS_NONE: u8 = 0x00000000;
const RAY_FLAGS_MASKED: u8 = 0x00000001;

#[derive(TypedBuilder)]
pub struct Ray {
    #[builder(default)]
    pub origin: Point,

    #[builder(default)]
    pub direction: Vector,

    #[builder(default=MachineEpsilon::epsilon(&1.0))]
    pub start: f32,

    #[builder(default=f32::INFINITY)]
    pub end: f32,

    #[builder(default)]
    pub time: f32,

    #[builder(default=RAY_FLAGS_NONE)]
    pub flags: u8,
}

impl Ray {
    pub fn get_direction_signs(&self) -> [i8; 3] {
        [
            self.direction.x.is_sign_negative() as i8,
            self.direction.y.is_sign_negative() as i8,
            self.direction.y.is_sign_negative() as i8,
        ]
    }

    pub fn update_range_with_epsilon(&mut self) {
        self.start += MachineEpsilon::epsilon(&self.origin);
        self.end -= MachineEpsilon::epsilon(&(self.origin + self.direction * self.end));
    }

    pub fn update(&mut self, origin: &Point, direction: &Vector) {
        self.origin = origin.clone();
        self.direction = direction.clone();
        self.start = MachineEpsilon::epsilon(&self.origin);
        self.end = f32::INFINITY;
    }

    pub fn update_with_time(&mut self, origin: &Point, direction: &Vector, time: f32) {
        self.update(origin, direction);
        self.time = time;
    }
}

impl Default for Ray {
    fn default() -> Self { Ray::builder().build() }
}

#[derive(Default)]
pub struct RayHit {
    pub t: f32,
    pub b1: f32,
    pub b2: f32,
    pub mesh_index: u8,
    pub triangle_index: u8,
}

impl RayHit {
    pub fn set_miss(&mut self) { self.mesh_index = 0xffffffff; }

    pub fn miss(&self) -> bool { self.mesh_index == 0xffffffff }
}
