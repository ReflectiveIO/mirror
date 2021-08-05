use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureMapping3D, TextureType};

pub struct BlenderVoronoiTexture {
    mapping: TextureMapping3D,
    intensity: i32,
    feature_weight1: f32,
    feature_weight2: f32,
    feature_weight3: f32,
    feature_weight4: f32,
    distance_metric: DistanceMetric,
    exponent: f32,
    noise_size: f32,
    bright: f32,
    contrast: f32,
}

impl BlenderVoronoiTexture {
    pub fn new(
        mapping: TextureMapping3D,
        intensity: i32,
        feature_weight1: f32,
        feature_weight2: f32,
        feature_weight3: f32,
        feature_weight4: f32,
        distance_metric: DistanceMetric,
        exponent: f32,
        noise_size: f32,
        bright: f32,
        contrast: f32,
    ) -> Self {
        Self {
            mapping,
            intensity,
            feature_weight1,
            feature_weight2,
            feature_weight3,
            feature_weight4,
            distance_metric,
            exponent,
            noise_size,
            bright,
            contrast,
        }
    }

    pub fn get_texture_mapping(&self) -> &TextureMapping3D { &self.mapping }

    pub fn get_intensity(&self) -> i32 { self.intensity }

    pub fn get_feature_weight1(&self) -> f32 { self.feature_weight1 }

    pub fn get_feature_weight2(&self) -> f32 { self.feature_weight2 }

    pub fn get_feature_weight3(&self) -> f32 { self.feature_weight3 }

    pub fn get_feature_weight4(&self) -> f32 { self.feature_weight4 }

    pub fn get_distance_metric(&self) -> &DistanceMetric { self.distance_metric }

    pub fn get_exponent(&self) -> f32 { self.exponent }

    pub fn get_noise_size(&self) -> f32 { self.noise_size }

    pub fn get_bright(&self) -> f32 { self.bright }

    pub fn get_contrast(&self) -> f32 { self.contrast }
}

impl Texture for BlenderVoronoiTexture {
    fn get_type(&self) -> TextureType { TextureType::BlenderVoronoi }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 0.5 }

    fn filter(&self) -> f32 { 0.5 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
