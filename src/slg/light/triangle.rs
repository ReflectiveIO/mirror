use std::collections::HashSet;
use std::f32::consts::PI;

use crate::rays::color::Spectrum;
use crate::rays::geometry::{Normal, Point, Ray};
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::bsdf::BSDF;
use crate::slg::image_map::ImageMap;
use crate::slg::light::{IntersectableLightSource, LightSource, LightSourceType};
use crate::slg::material::Material;
use crate::slg::scene::SceneObject;
use crate::slg::Scene;

pub struct TriangleLight {
    pub light_material: Material,
    pub scene_object: SceneObject,

    // Note: mesh_index is initialized in
    // LightSourceDefinitions::preprocess()
    pub mesh_index: u32,
    pub triangle_index: u32,

    triangle_area: f32,
    inv_triangle_area: f32,
    mesh_area: f32,
    inv_mesh_area: f32,
}

impl TriangleLight {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_triangle_area(&self) -> f32 {
        self.triangle_area
    }

    pub fn get_mesh_area(&self) -> f32 {
        self.mesh_area
    }

    pub fn get_area(&self) -> f32 {
        self.triangle_area
    }
}

impl IntersectableLightSource for TriangleLight {
    fn preprocess(&self) {
        todo!()
    }

    fn get_type(&self) -> LightSourceType {
        LightSourceType::TriangleLight
    }

    fn get_power(&self, scene: &Scene) -> f32 {
        let emitted_radiance_y: f32 = self
            .light_material
            .get_emitted_radiance_y(self.inv_mesh_area);

        if self.light_material.get_emitted_theta == 0.0 {
            self.triangle_area * emitted_radiance_y
        } else if self.light_material.get_emitted_theta < 90.0 {
            self.triangle_area
                * (2.0 * PI)
                * (1.0 - self.light_material.get_emitted_cos_theta_max())
                * emitted_radiance_y
        } else {
            self.triangle_area * PI * emitted_radiance_y
        }
    }

    fn is_direct_light_sampling_enabled(&self) -> bool {
        match self.light_material.get_direct_light_sampling_type() {
            DLS_AUTO => {
                if self.scene_object.get_ext_mesh().get_total_triangle_count > 256 {
                    false
                } else {
                    true
                }
            }
            DLS_ENABLED => true,
            DLS_DISABLED => false,
        }
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

    fn is_always_in_shadow(&self, scene: &Scene, p: &Point, n: &Normal) -> bool {
        todo!()
    }

    fn get_radiance(
        &self,
        hit_point: &HitPoint,
        direct_pd_fa: Option<Vec<f32>>,
        emission_pd_fw: Option<Vec<f32>>,
    ) {
        todo!()
    }

    fn light_material(&self) -> &Material {
        &self.light_material
    }
}
