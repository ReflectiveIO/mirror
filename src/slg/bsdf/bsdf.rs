use crate::rays::color::Spectrum;
use crate::rays::geometry::{Frame, Point, Ray, RayHit, Vector};
use crate::rays::mesh::{ExtMesh, ExtTriangleMesh};
use crate::slg::bsdf::{BSDFEvent, HitPoint};
use crate::slg::light::traits::LightSource;
use crate::slg::light::TriangleLight;
use crate::slg::material::{MaterialTrait, MaterialType};
use crate::slg::scene::{BakeMapType, SceneObject};
use crate::slg::utils::PathVolumeInfo;
use crate::slg::volume::Volume;
use crate::slg::Scene;

pub struct BSDF {
    pub hit_point: HitPoint,
    material: Option<Box<dyn MaterialTrait>>,
    scene_object: SceneObject,
    mesh: Box<dyn ExtMesh>,
    triangle_light_source: TriangleLight,
    frame: Frame,
}

impl Default for BSDF {
    fn default() -> Self { Self::empty() }
}

impl BSDF {
    /// An Empty BSDF
    pub fn empty() -> Self {
        Self {
            hit_point: HitPoint::default(),
            material: None,
            scene_object: SceneObject::default(),
            mesh: Box::new(ExtTriangleMesh::default()),
            triangle_light_source: TriangleLight::new(),
            frame: Frame::default(),
        }
    }

    /// A BSDF initialized from a ray hit
    pub fn from_ray_hit(
        fixed_from_light: bool,
        through_shadow_transparency: bool,
        scene: &Scene,
        ray: &Ray,
        ray_hit: &RayHit,
        pass_through_event: f32,
        vol_info: &PathVolumeInfo,
    ) -> Self {
        todo!()
    }

    /// A BSDF initialized with point on a surface
    pub fn from_point(
        scene: &Scene,
        mesh_index: u32,
        triangle_index: u32,
        surface_point: &Point,
        surface_point_bary1: f32,
        surface_point_bary2: f32,
        time: f32,
        pass_through_event: f32,
        vol_info: &PathVolumeInfo,
    ) -> Self {
        todo!()
    }

    /// A BSDF initialized with a volume scattering point
    pub fn from_volume(
        fixed_from_light: bool,
        through_shadow_transparency: bool,
        scene: &Scene,
        ray: &Ray,
        volume: &Volume,
        t: f32,
        pass_through_event: f32,
    ) -> Self {
        todo!()
    }

    pub fn is_empty(&self) -> bool { self.material.is_none() }

    pub fn is_light_source(&self) -> bool { self.material.unwrap().is_light_source() }

    pub fn is_delta(&self) -> bool { self.material.unwrap().is_delta() }

    pub fn is_visible_indirect_diffuse(&self) -> bool {
        self.material.unwrap().is_visible_indirect_diffuse()
    }

    pub fn is_visible_indirect_glossy(&self) -> bool {
        self.material.unwrap().is_visible_indirect_glossy()
    }

    pub fn is_visible_indirect_specular(&self) -> bool {
        self.material.unwrap().is_visible_indirect_specular()
    }

    pub fn is_shadow_catcher(&self) -> bool { self.material.unwrap().is_shadow_catcher() }

    pub fn is_shadow_catcher_only_infinite_lights(&self) -> bool {
        self.material
            .unwrap()
            .is_shadow_catcher_only_infinite_lights()
    }

    pub fn is_camera_invisible(&self) -> bool { todo!() }

    pub fn is_volume(&self) -> bool { todo!() }

    pub fn is_photon_gi_enabled(&self) -> bool { self.material.unwrap().is_photon_gi_enabled() }

    pub fn is_holdout(&self) -> bool { self.material.unwrap().is_holdout() }

    pub fn is_albedo_endpoint(&self) -> bool { todo!() }

    pub fn get_object_id(&self) -> u32 { todo!() }

    pub fn get_material_name(&self) -> &String { todo!() }

    pub fn get_material_id(&self) -> u32 { self.material.unwrap().get_id() }

    pub fn get_light_id(&self) -> u32 { self.material.unwrap().get_light_id() }

    pub fn get_material_interior_volume(&self) -> &Volume {
        self.material
            .unwrap()
            .get_interior_volume(&self.hit_point, self.hit_point.pass_through_event)
    }

    pub fn get_material_exterior_volume(&self) -> &Volume {
        self.material
            .unwrap()
            .get_exterior_volume(&self.hit_point, self.hit_point.pass_through_event)
    }

    pub fn get_glossiness(&self) -> f32 { self.material.unwrap().get_glossiness() }

    pub fn get_scene_object(&self) -> &SceneObject { &self.scene_object }

    pub fn get_event_types(&self) -> BSDFEvent { self.material.unwrap().get_event_types() }

    pub fn get_material_type(&self) -> MaterialType { self.material.unwrap().get_type() }

    pub fn get_pass_through_transparency(&self, back_tracing: bool) -> Spectrum { todo!() }

    pub fn get_pass_through_shadow_transparency(&self) -> &Spectrum {
        self.material
            .unwrap()
            .get_pass_through_shadow_transparency()
    }

    pub fn get_frame(&self) -> &Frame { &self.frame }

    pub fn albedo(&self) -> Spectrum { todo!() }

    pub fn evaluate_total(&self) -> Spectrum { todo!() }

    pub fn evaluate(
        &self,
        generated_dir: &Vector,
        event: &BSDFEvent,
        direct_pdf_w: Option<f32>,
        reverse_pdf_w: Option<f32>,
    ) -> Spectrum {
        todo!()
    }

    pub fn sample(
        &self,
        sampled_dir: &Vector,
        u0: f32,
        u1: f32,
        pdf_w: f32,
        abs_cos_sampled_dir: f32,
        event: &BSDFEvent,
    ) -> Spectrum {
        todo!()
    }

    pub fn shadow_catcher_sample(
        &self,
        sampled_dir: &Vector,
        pdf_w: f32,
        abs_cos_sampled_dir: f32,
        event: &BSDFEvent,
    ) -> Spectrum {
        todo!()
    }

    pub fn pdf(&self, sampled_dir: &Vector, direct_pdf_w: f32, reverse_pdf_w: f32) { todo!() }

    pub fn get_emitted_radiance(
        &self,
        direct_pdf_w: Option<f32>,
        emission_pdf_w: Option<f32>,
    ) -> Spectrum {
        todo!()
    }

    pub fn get_light_source(&self) -> &dyn LightSource { &self.triangle_light_source }

    pub fn get_ray_origin(&self, sample_dir: &Vector) -> Point { todo!() }

    pub fn has_bake_map(&self, t: BakeMapType) -> bool { todo!() }

    pub fn get_bake_map_value(&self) -> Spectrum { todo!() }
}
