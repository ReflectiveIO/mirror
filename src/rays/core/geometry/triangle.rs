use crate::rays::core::geometry::Point;
use crate::rays::geometry::Cross;

#[derive(Default)]
pub struct Triangle {
    pub v: [usize; 3],
}

impl Triangle {
    pub fn area(&self, vertices: &Vec<Point>) -> f32 { todo!() }

    pub fn sample(
        &self,
        vertices: &Vec<Point>,
        u0: f32,
        u1: f32,
        p: &Point,
        b0: f32,
        b1: f32,
        b2: f32,
    ) {
        todo!()
    }
}

pub fn area(p0: &Point, p1: &Point, p2: &Point) -> f32 {
    0.5 * (p1 - p0).cross(&(p2 - p0)).length()
}
