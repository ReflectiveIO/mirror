use std::collections::HashSet;

use downcast_rs::Downcast;

use crate::rays::color::Spectrum;
use crate::rays::geometry::{Normal, Point, Ray, Transform};
use crate::rays::Properties;
use crate::slg::bsdf::BSDF;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::light::traits::LightSource;
use crate::slg::light::LightSourceType;
use crate::slg::Scene;

pub trait NotIntersectableLightSource: LightSource + Downcast {
    fn preprocess(&self);
    fn get_type(&self) -> LightSourceType;

    fn is_environmental(&self) -> bool { false }

    fn is_infinite(&self) -> bool { false }

    fn is_intersectable(&self) -> bool { false }

    fn get_avg_pass_through_transparency(&self) -> f32 { 1.0 }

    fn set_id(&mut self, light_id: u32);
    fn get_id(&self) -> u32;

    fn get_power(&self, scene: &Scene) -> f32;

    fn get_importance(&self) -> f32;
    fn set_importance(&self, imp: f32) -> f32;

    fn is_direct_light_sampling_enabled(&self) -> bool;

    fn is_visible_indirect_diffuse(&self) -> bool { false }

    fn is_visible_indirect_glossy(&self) -> bool { false }

    fn is_visible_indirect_specular(&self) -> bool { false }

    fn emit(
        &mut self,
        scene: &Scene,
        time: f32,
        u0: f32,
        u1: f32,
        u2: f32,
        u3: f32,
        pass_through_event: f32,
        ray: &Ray,
        emission_pd_fw: f32,
        direct_pd_fa: Option<Vec<f32>>,
        cos_theta_at_light: Option<Vec<f32>>,
    ) -> Spectrum;

    fn illuminate(
        &mut self,
        scene: &Scene,
        bsdf: &BSDF,
        time: f32,
        u0: f32,
        u1: f32,
        pass_through_event: f32,
        shadow_ray: &Ray,
        direct_pd_fw: f32,
        emission_pd_fw: Option<Vec<f32>>,
        cos_theta_at_light: Option<Vec<f32>>,
    ) -> Spectrum;

    fn is_always_in_shadow(&self, scene: &Scene, p: &Point, n: &Normal) -> bool;

    fn add_referenced_image_maps(&mut self, maps: HashSet<ImageMap>) {}

    fn temperature_scale(&self) -> &Spectrum;
    fn light_to_world(&self) -> &Transform;
    fn gain(&self) -> &Spectrum;
    fn temperature(&self) -> f32;
    fn normalize_temperature(&self) -> bool;
    fn to_properties(&self, img_map_cache: &ImageMapCache, real_filename: bool) -> Properties;
}
impl_downcast!(NotIntersectableLightSource);
