use crate::rays::geometry::Point;

pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Bounds {
    pub fn new() -> Self {
        Self {
            min: Point::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Point::new(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY),
        }
    }
}

impl From<&Point> for Bounds {
    fn from(val: &Point) -> Self {
        Self {
            min: val.clone(),
            max: val.clone(),
        }
    }
}

impl From<(&Point, &Point)> for Bounds {
    fn from((p1, p2): (&Point, &Point)) -> Self {
        Self {
            min: Point::new(
                f32::min(p1.x, p2.x),
                f32::min(p1.y, p2.y),
                f32::min(p1.z, p2.z),
            ),
            max: Point::new(
                f32::max(p1.x, p2.x),
                f32::max(p1.y, p2.y),
                f32::max(p1.z, p2.z),
            ),
        }
    }
}
