#[derive(Default)]
pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[inline]
pub fn dot(v1: &Normal, v2: &Normal) -> f32 { v1.x * v2.x + v1.y * v2.y + v1.z * v2.z }
