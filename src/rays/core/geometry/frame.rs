use crate::rays::geometry::{coordinate_system, Cross, Dot, Normal, Vector};

pub struct Frame {
    pub x: Vector,
    pub y: Vector,
    pub z: Vector,
}

impl Frame {
    pub fn new(x: &Vector, y: &Vector, z: &Normal) -> Self {
        let z = Vector::from(z.clone());
        let y = z.cross(x).normalize();
        let x = y.cross(&z);

        Self { x, y, z }
    }

    pub fn set_from_z(&mut self, z: &Vector) {
        self.z = z.clone();
        coordinate_system(&self.z, &mut self.x, &mut self.y)
    }

    pub fn to_world(&self, a: &Vector) -> Vector { self.x * a.x + self.y * a.y + self.z * a.z }

    pub fn to_local(&self, a: &Vector) -> Vector {
        Vector::new(a.dot(&self.x), a.dot(&self.y), a.dot(&self.z))
    }

    pub fn binormal(&self) -> &Vector { &self.x }

    pub fn tangent(&self) -> &Vector { &self.y }

    pub fn normal(&self) -> &Vector { &self.z }
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            x: Vector::new(1.0, 0.0, 0.0),
            y: Vector::new(0.0, 1.0, 0.0),
            z: Vector::new(0.0, 0.0, 1.0),
        }
    }
}

impl From<&Vector> for Frame {
    fn from(val: &Vector) -> Self {
        let mut frame = Self::default();
        frame.set_from_z(val);
        return frame;
    }
}

impl From<&Normal> for Frame {
    fn from(val: &Normal) -> Self {
        let mut frame = Self::default();
        frame.set_from_z(&Vector::from(val.clone()));
        return frame;
    }
}
