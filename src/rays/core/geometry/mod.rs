pub use self::bsphere::BSphere;
pub use self::frame::Frame;
pub use self::matrix4x4::Matrix4x4;
pub use self::motion::MotionSystem;
pub use self::normal::Normal;
pub use self::point::Point;
pub use self::ray::{Ray, RayHit};
pub use self::transform::*;
pub use self::triangle::Triangle;
pub use self::uv::UV;
pub use self::vector::*;

mod bsphere;
mod frame;
mod matrix4x4;
mod motion;
mod normal;
mod point;
mod ray;
mod transform;
mod triangle;
mod uv;
mod vector;

pub trait Cross<Rhs = Self> {
    type Output;
    fn cross(&self, rhs: &Rhs) -> Self::Output;
}

pub trait Dot<Rhs = Self> {
    fn dot(&self, rhs: &Rhs) -> f32;
}
