use std::collections::HashSet;

use crate::rays::color::Spectrum;
use crate::rays::geometry::{Normal, Point, Ray, Transform, Vector, UV};
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::bsdf::BSDF;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::material::{Material, MaterialTrait};
use crate::slg::Scene;

pub enum LightSourceType {
    Infinite,
    Sky,
    Sun,
    TriangleLight,
    Point,
    MapPoint,
    Spotlight,
    Projection,
    ConstantInfinite,
    SharpDistant,
    Distant,
    Sky2,
    Laser,
    Sphere,
    MapSphere,
}
