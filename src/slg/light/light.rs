use std::collections::HashSet;

use crate::rays::color::Spectrum;
use crate::rays::geometry::{Normal, Point, Ray, Transform, Vector, UV};
use crate::rays::{NamedObject, Properties};
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::bsdf::BSDF;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::material::Material;
use crate::slg::Scene;

pub trait LightSource {
    fn preprocess(&self);
    fn get_type(&self) -> LightSourceType;

    fn is_environmental(&self) -> bool {
        false
    }

    fn is_infinite(&self) -> bool {
        false
    }

    fn is_intersectable(&self) -> bool {
        false
    }

    fn get_avg_pass_through_transparency(&self) -> f32 {
        1.0
    }
    fn get_id(&self) -> u32;
    fn get_power(&self, scene: &Scene) -> f32;
    fn get_importance(&self) -> f32;

    fn is_direct_light_sampling_enabled(&self) -> bool;

    fn is_visible_indirect_diffuse(&self) -> bool;
    fn is_visible_indirect_glossy(&self) -> bool;
    fn is_visible_indirect_specular(&self) -> bool;

    /// Emits particle from the light
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

    /// Illuminates bsdf.hitPoint.p
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

    // This can be used at pre-processing stage to check if the point is always in
    // shadow (to avoid tracing the shadow ray). This method can be optionally
    // implemented by a light source. The return value can be just false if the
    // answer is unknown.
    fn is_always_in_shadow(&self, scene: &Scene, p: &Point, n: &Normal) -> bool {
        false
    }

    fn add_referenced_image_maps(&mut self, maps: HashSet<ImageMap>) {}
}

/// Intersectable LightSource interface
pub trait IntersectableLightSource {
    fn preprocess(&self);
    fn get_type(&self) -> LightSourceType;

    fn is_environmental(&self) -> bool {
        false
    }

    fn is_infinite(&self) -> bool {
        false
    }

    fn is_intersectable(&self) -> bool {
        true
    }

    fn get_avg_pass_through_transparency(&self) -> f32 {
        self.light_material().get_avg_pass_through_transparency()
    }

    fn get_id(&self) -> u32 {
        self.light_material().get_light_id()
    }

    fn get_power(&self, scene: &Scene) -> f32;

    fn get_importance(&self) -> f32 {
        self.light_material().get_emitted_importance()
    }

    fn is_direct_light_sampling_enabled(&self) -> bool;

    fn is_visible_indirect_diffuse(&self) -> bool {
        self.light_material().is_visible_indirect_diffuse()
    }

    fn is_visible_indirect_glossy(&self) -> bool {
        self.light_material().is_visible_indirect_glossy()
    }

    fn is_visible_indirect_specular(&self) -> bool {
        self.light_material().is_visible_indirect_specular()
    }

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

    fn get_radiance(
        &self,
        hit_point: &HitPoint,
        direct_pd_fa: Option<Vec<f32>>,
        emission_pd_fw: Option<Vec<f32>>,
    );

    fn light_material(&self) -> &Material;
}

// impl<T: IntersectableLightSource> LightSource for T {
//     fn preprocess(&self) {
//         self.preprocess()
//     }
//
//     fn get_type(&self) -> LightSourceType {
//         self.get_type()
//     }
//
//     fn is_environmental(&self) -> bool {
//         self.is_environmental()
//     }
//
//     fn is_infinite(&self) -> bool {
//         self.is_infinite()
//     }
//
//     fn is_intersectable(&self) -> bool {
//         self.is_intersectable()
//     }
//
//     fn get_avg_pass_through_transparency(&self) -> f32 {
//         self.get_avg_pass_through_transparency()
//     }
//
//     fn get_id(&self) -> u32 {
//         self.get_id()
//     }
//
//     fn get_power(&self, scene: &Scene) -> f32 {
//         self.get_power(scene)
//     }
//
//     fn get_importance(&self) -> f32 {
//         self.get_importance()
//     }
//
//     fn is_direct_light_sampling_enabled(&self) -> bool {
//         self.is_direct_light_sampling_enabled()
//     }
//
//     fn is_visible_indirect_diffuse(&self) -> bool {
//         self.is_visible_indirect_diffuse()
//     }
//
//     fn is_visible_indirect_glossy(&self) -> bool {
//         self.is_visible_indirect_glossy()
//     }
//
//     fn is_visible_indirect_specular(&self) -> bool {
//         self.is_visible_indirect_specular()
//     }
//
//     fn Emit(
//         &mut self,
//         scene: &Scene,
//         time: f32,
//         u0: f32,
//         u1: f32,
//         u2: f32,
//         u3: f32,
//         pass_through_event: f32,
//         ray: &Ray,
//         emission_pd_fw: f32,
//         direct_pd_fa: Option<Vec<f32>>,
//         cos_theta_at_light: Option<Vec<f32>>,
//     ) -> Spectrum {
//         self.Emit(
//             scene,
//             time,
//             u0,
//             u1,
//             u2,
//             u3,
//             pass_through_event,
//             ray,
//             emission_pd_fw,
//             direct_pd_fa,
//             cos_theta_at_light,
//         )
//     }
//
//     fn illuminate(
//         &mut self,
//         scene: &Scene,
//         bsdf: &BSDF,
//         time: f32,
//         u0: f32,
//         u1: f32,
//         pass_through_event: f32,
//         shadow_ray: &Ray,
//         direct_pd_fw: f32,
//         emission_pd_fw: Option<Vec<f32>>,
//         cos_theta_at_light: Option<Vec<f32>>,
//     ) -> Spectrum {
//         self.illuminate(
//             scene,
//             bsdf,
//             time,
//             u0,
//             u1,
//             pass_through_event,
//             shadow_ray,
//             direct_pd_fw,
//             emission_pd_fw,
//             cos_theta_at_light,
//         )
//     }
//
//     fn is_always_in_shadow(&self, scene: &Scene, p: &Point, n: &Normal) -> bool {
//         self.is_always_in_shadow(scene, p, n)
//     }
//
//     fn add_referenced_image_maps(&mut self, maps: HashSet<ImageMap>) {
//         self.add_referenced_image_maps(maps)
//     }
// }

pub trait NotIntersectableLightSource {
    fn preprocess(&self);
    fn get_type(&self) -> LightSourceType;

    fn is_environmental(&self) -> bool {
        false
    }

    fn is_infinite(&self) -> bool {
        false
    }

    fn is_intersectable(&self) -> bool {
        false
    }

    fn get_avg_pass_through_transparency(&self) -> f32 {
        1.0
    }

    fn set_id(&mut self, light_id: u32);
    fn get_id(&self) -> u32;

    fn get_power(&self, scene: &Scene) -> f32;

    fn get_importance(&self) -> f32;
    fn set_importance(&self, imp: f32) -> f32;

    fn is_direct_light_sampling_enabled(&self) -> bool;

    fn is_visible_indirect_diffuse(&self) -> bool {
        false
    }

    fn is_visible_indirect_glossy(&self) -> bool {
        false
    }

    fn is_visible_indirect_specular(&self) -> bool {
        false
    }

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

// impl<T: NotIntersectableLightSource> LightSource for T {
//     fn preprocess(&self) {
//         self.preprocess()
//     }
//
//     fn get_type(&self) -> LightSourceType {
//         self.get_type()
//     }
//
//     fn is_environmental(&self) -> bool {
//         self.is_environmental()
//     }
//
//     fn is_infinite(&self) -> bool {
//         self.is_infinite()
//     }
//
//     fn is_intersectable(&self) -> bool {
//         self.is_intersectable()
//     }
//
//     fn get_avg_pass_through_transparency(&self) -> f32 {
//         self.get_avg_pass_through_transparency()
//     }
//
//     fn get_id(&self) -> u32 {
//         self.get_id()
//     }
//
//     fn get_power(&self, scene: &Scene) -> f32 {
//         self.get_power(scene)
//     }
//
//     fn get_importance(&self) -> f32 {
//         self.get_importance()
//     }
//
//     fn is_direct_light_sampling_enabled(&self) -> bool {
//         self.is_direct_light_sampling_enabled()
//     }
//
//     fn is_visible_indirect_diffuse(&self) -> bool {
//         self.is_visible_indirect_diffuse()
//     }
//
//     fn is_visible_indirect_glossy(&self) -> bool {
//         self.is_visible_indirect_glossy()
//     }
//
//     fn is_visible_indirect_specular(&self) -> bool {
//         self.is_visible_indirect_specular()
//     }
//
//     fn Emit(
//         &mut self,
//         scene: &Scene,
//         time: f32,
//         u0: f32,
//         u1: f32,
//         u2: f32,
//         u3: f32,
//         pass_through_event: f32,
//         ray: &Ray,
//         emission_pd_fw: f32,
//         direct_pd_fa: Option<Vec<f32>>,
//         cos_theta_at_light: Option<Vec<f32>>,
//     ) -> Spectrum {
//         self.Emit(
//             scene,
//             time,
//             u0,
//             u1,
//             u2,
//             u3,
//             pass_through_event,
//             ray,
//             emission_pd_fw,
//             direct_pd_fa,
//             cos_theta_at_light,
//         )
//     }
//
//     fn illuminate(
//         &mut self,
//         scene: &Scene,
//         bsdf: &BSDF,
//         time: f32,
//         u0: f32,
//         u1: f32,
//         pass_through_event: f32,
//         shadow_ray: &Ray,
//         direct_pd_fw: f32,
//         emission_pd_fw: Option<Vec<f32>>,
//         cos_theta_at_light: Option<Vec<f32>>,
//     ) -> Spectrum {
//         self.illuminate(
//             scene,
//             bsdf,
//             time,
//             u0,
//             u1,
//             pass_through_event,
//             shadow_ray,
//             direct_pd_fw,
//             emission_pd_fw,
//             cos_theta_at_light,
//         )
//     }
//
//     fn is_always_in_shadow(&self, scene: &Scene, p: &Point, n: &Normal) -> bool {
//         self.is_always_in_shadow(scene, p, n)
//     }
//
//     fn add_referenced_image_maps(&mut self, maps: HashSet<ImageMap>) {
//         self.add_referenced_image_maps(maps)
//     }
// }

pub trait InfiniteLightSource {
    fn preprocess(&self);
    fn get_type(&self) -> LightSourceType;

    fn is_environmental(&self) -> bool {
        false
    }

    fn is_infinite(&self) -> bool {
        true
    }

    fn is_intersectable(&self) -> bool {
        false
    }

    fn get_avg_pass_through_transparency(&self) -> f32 {
        1.0
    }

    fn set_id(&mut self, light_id: u32);
    fn get_id(&self) -> u32;

    fn get_power(&self, scene: &Scene) -> f32;

    fn get_importance(&self) -> f32;
    fn set_importance(&self, imp: f32) -> f32;

    fn set_indirect_diffuse_visibility(&mut self, visible: bool);
    fn set_indirect_glossy_visibility(&mut self, visible: bool);
    fn set_indirect_specular_visibility(&mut self, visible: bool);

    fn is_direct_light_sampling_enabled(&self) -> bool;
    fn is_visible_indirect_diffuse(&self) -> bool;
    fn is_visible_indirect_glossy(&self) -> bool;
    fn is_visible_indirect_specular(&self) -> bool;

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

// impl<T: InfiniteLightSource> LightSource for T {
//     fn preprocess(&self) {
//         self.preprocess()
//     }
//
//     fn get_type(&self) -> LightSourceType {
//         self.get_type()
//     }
//
//     fn is_environmental(&self) -> bool {
//         self.is_environmental()
//     }
//
//     fn is_infinite(&self) -> bool {
//         self.is_infinite()
//     }
//
//     fn is_intersectable(&self) -> bool {
//         self.is_intersectable()
//     }
//
//     fn get_avg_pass_through_transparency(&self) -> f32 {
//         self.get_avg_pass_through_transparency()
//     }
//
//     fn get_id(&self) -> u32 {
//         self.get_id()
//     }
//
//     fn get_power(&self, scene: &Scene) -> f32 {
//         self.get_power(scene)
//     }
//
//     fn get_importance(&self) -> f32 {
//         self.get_importance()
//     }
//
//     fn is_direct_light_sampling_enabled(&self) -> bool {
//         self.is_direct_light_sampling_enabled()
//     }
//
//     fn is_visible_indirect_diffuse(&self) -> bool {
//         self.is_visible_indirect_diffuse()
//     }
//
//     fn is_visible_indirect_glossy(&self) -> bool {
//         self.is_visible_indirect_glossy()
//     }
//
//     fn is_visible_indirect_specular(&self) -> bool {
//         self.is_visible_indirect_specular()
//     }
//
//     fn Emit(
//         &mut self,
//         scene: &Scene,
//         time: f32,
//         u0: f32,
//         u1: f32,
//         u2: f32,
//         u3: f32,
//         pass_through_event: f32,
//         ray: &Ray,
//         emission_pd_fw: f32,
//         direct_pd_fa: Option<Vec<f32>>,
//         cos_theta_at_light: Option<Vec<f32>>,
//     ) -> Spectrum {
//         self.Emit(
//             scene,
//             time,
//             u0,
//             u1,
//             u2,
//             u3,
//             pass_through_event,
//             ray,
//             emission_pd_fw,
//             direct_pd_fa,
//             cos_theta_at_light,
//         )
//     }
//
//     fn illuminate(
//         &mut self,
//         scene: &Scene,
//         bsdf: &BSDF,
//         time: f32,
//         u0: f32,
//         u1: f32,
//         pass_through_event: f32,
//         shadow_ray: &Ray,
//         direct_pd_fw: f32,
//         emission_pd_fw: Option<Vec<f32>>,
//         cos_theta_at_light: Option<Vec<f32>>,
//     ) -> Spectrum {
//         self.illuminate(
//             scene,
//             bsdf,
//             time,
//             u0,
//             u1,
//             pass_through_event,
//             shadow_ray,
//             direct_pd_fw,
//             emission_pd_fw,
//             cos_theta_at_light,
//         )
//     }
//
//     fn is_always_in_shadow(&self, scene: &Scene, p: &Point, n: &Normal) -> bool {
//         self.is_always_in_shadow(scene, p, n)
//     }
//
//     fn add_referenced_image_maps(&mut self, maps: HashSet<ImageMap>) {
//         self.add_referenced_image_maps(maps)
//     }
// }

pub trait EnvLightSource {
    fn preprocess(&self);
    fn get_type(&self) -> LightSourceType;

    fn is_environmental(&self) -> bool {
        true
    }

    fn is_infinite(&self) -> bool {
        true
    }

    fn is_intersectable(&self) -> bool {
        false
    }

    fn get_avg_pass_through_transparency(&self) -> f32 {
        1.0
    }

    fn set_id(&mut self, light_id: u32);
    fn get_id(&self) -> u32;

    fn get_power(&self, scene: &Scene) -> f32;

    fn get_importance(&self) -> f32;
    fn set_importance(&self, imp: f32) -> f32;

    fn set_indirect_diffuse_visibility(&mut self, visible: bool);
    fn set_indirect_glossy_visibility(&mut self, visible: bool);
    fn set_indirect_specular_visibility(&mut self, visible: bool);

    fn is_direct_light_sampling_enabled(&self) -> bool;
    fn is_visible_indirect_diffuse(&self) -> bool;
    fn is_visible_indirect_glossy(&self) -> bool;
    fn is_visible_indirect_specular(&self) -> bool;

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

    fn get_env_uv(&self, dir: &Vector) -> UV {
        panic!("Internal error: called EnvLightSource::get_env_uv()")
    }

    fn update_visibility_map(&mut self, scene: &Scene, real_time: bool) {}

    // Note: bsdf parameter can be null if it is a camera ray
    fn get_radiance(
        &self,
        scene: &Scene,
        bsdf: &BSDF,
        dir: &Vector,
        direct_pd_fa: Option<Vec<f32>>,
        emission_pd_fw: Option<Vec<f32>>,
    ) -> Spectrum;
}

impl<T> LightSource for T
where
    T: IntersectableLightSource,
    T: NotIntersectableLightSource,
    T: InfiniteLightSource,
    T: EnvLightSource,
{
    fn preprocess(&self) {
        self.preprocess()
    }

    fn get_type(&self) -> LightSourceType {
        self.get_type()
    }

    fn is_environmental(&self) -> bool {
        self.is_environmental()
    }

    fn is_infinite(&self) -> bool {
        self.is_infinite()
    }

    fn is_intersectable(&self) -> bool {
        self.is_intersectable()
    }

    fn get_avg_pass_through_transparency(&self) -> f32 {
        self.get_avg_pass_through_transparency()
    }

    fn get_id(&self) -> u32 {
        self.get_id()
    }

    fn get_power(&self, scene: &Scene) -> f32 {
        self.get_power(scene)
    }

    fn get_importance(&self) -> f32 {
        self.get_importance()
    }

    fn is_direct_light_sampling_enabled(&self) -> bool {
        self.is_direct_light_sampling_enabled()
    }

    fn is_visible_indirect_diffuse(&self) -> bool {
        self.is_visible_indirect_diffuse()
    }

    fn is_visible_indirect_glossy(&self) -> bool {
        self.is_visible_indirect_glossy()
    }

    fn is_visible_indirect_specular(&self) -> bool {
        self.is_visible_indirect_specular()
    }

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
        self.emit(
            scene,
            time,
            u0,
            u1,
            u2,
            u3,
            pass_through_event,
            ray,
            emission_pd_fw,
            direct_pd_fa,
            cos_theta_at_light,
        )
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
        self.illuminate(
            scene,
            bsdf,
            time,
            u0,
            u1,
            pass_through_event,
            shadow_ray,
            direct_pd_fw,
            emission_pd_fw,
            cos_theta_at_light,
        )
    }

    fn is_always_in_shadow(&self, scene: &Scene, p: &Point, n: &Normal) -> bool {
        self.is_always_in_shadow(scene, p, n)
    }

    fn add_referenced_image_maps(&mut self, maps: HashSet<ImageMap>) {
        self.add_referenced_image_maps(maps)
    }
}

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
