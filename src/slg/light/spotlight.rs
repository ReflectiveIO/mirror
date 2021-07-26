use crate::rays::color::Spectrum;
use crate::rays::geometry::{Normal, Point, Ray, Transform};
use crate::rays::Properties;
use crate::slg::bsdf::BSDF;
use crate::slg::image_map::ImageMapCache;
use crate::slg::light::traits::{LightSource, NotIntersectableLightSource};
use crate::slg::light::LightSourceType;
use crate::slg::Scene;

pub struct Spotlight {
    pub color: Spectrum,
    pub power: f32,
    pub efficiency: f32,
    pub emitted_power_normalize: bool,
    pub cone_angle: f32,
    pub cone_delta_angle: f32,

    emitted_factor: Spectrum,
    absolute_point: Point,
    cos_total_width: f32,
    cos_falloff_start: f32,
    aligned_light2world: Transform,
}

impl LightSource for Spotlight {
    fn get_type(&self) -> LightSourceType { todo!() }

    fn preprocess(&self) { todo!() }

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
    ) -> Spectrum {
        todo!()
    }

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
    ) -> Spectrum {
        todo!()
    }
}

impl NotIntersectableLightSource for Spotlight {
    fn preprocess(&self) { todo!() }

    fn get_type(&self) -> LightSourceType { todo!() }

    fn set_id(&mut self, light_id: u32) { todo!() }

    fn get_id(&self) -> u32 { todo!() }

    fn get_power(&self, scene: &Scene) -> f32 { todo!() }

    fn get_importance(&self) -> f32 { todo!() }

    fn set_importance(&self, imp: f32) -> f32 { todo!() }

    fn is_direct_light_sampling_enabled(&self) -> bool { todo!() }

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
    ) -> Spectrum {
        todo!()
    }

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
    ) -> Spectrum {
        todo!()
    }

    fn is_always_in_shadow(&self, scene: &Scene, p: &Point, n: &Normal) -> bool { todo!() }

    fn temperature_scale(&self) -> &Spectrum { todo!() }

    fn light_to_world(&self) -> &Transform { todo!() }

    fn gain(&self) -> &Spectrum { todo!() }

    fn temperature(&self) -> f32 { todo!() }

    fn normalize_temperature(&self) -> bool { todo!() }

    fn to_properties(&self, img_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
