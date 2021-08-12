use super::material::Material;
use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::Properties;
use crate::slg::bsdf::{BSDFEvent, BSDFEventType, HitPoint};
use crate::slg::image_map::ImageMapCache;
use crate::slg::material::base::BaseMaterial;
use crate::slg::material::MaterialType;
use crate::slg::textures::Texture;

pub struct NullMaterial {
    base: BaseMaterial,
}

impl NullMaterial {
    pub fn new(front_transp: &Box<dyn Texture>, back_transp: &Box<dyn Texture>) -> Self {
        Self {
            base: BaseMaterial::default(),
        }
    }
}

impl Default for NullMaterial {
    fn default() -> Self {
        Self {
            base: BaseMaterial::default(),
        }
    }
}

impl Material for NullMaterial {
    fn base(&self) -> &BaseMaterial { &self.base }

    fn base_mut(&mut self) -> &mut BaseMaterial { &mut self.base }

    fn get_type(&self) -> MaterialType { MaterialType::NullMat }

    fn get_event_types(&self) -> BSDFEvent { BSDFEventType::SPECULAR | BSDFEventType::TRANSMIT }

    fn is_delta(&self) -> bool { todo!() }

    fn get_pass_through_transparency(
        &self,
        hit_point: &HitPoint,
        local_fixed_dir: Vector,
        pass_through_event: f32,
        back_tracing: bool,
    ) -> Spectrum {
        todo!()
    }

    fn albedo(&self, hit_point: &HitPoint) -> Spectrum { todo!() }

    fn evaluate(
        &self,
        hp: &HitPoint,
        local_light_dir: &Vector,
        local_eye_dir: &Vector,
        event: &BSDFEvent,
        direct_pd_fw: Option<f32>,
        reverse_pd_fw: Option<f32>,
    ) -> Spectrum {
        todo!()
    }

    fn sample(
        &self,
        hp: &HitPoint,
        local_fixed_dir: &Vector,
        local_sample_dir: &Vector,
        u0: f32,
        u1: f32,
        pass_through_event: f32,
        pdfw: f32,
        event: &BSDFEvent,
    ) -> Spectrum {
        todo!()
    }

    fn pdf(
        &self,
        hp: &HitPoint,
        local_light_dir: &Vector,
        local_eye_dir: &Vector,
        direct_pd_fw: f32,
        reverse_pd_fw: f32,
    ) {
        todo!()
    }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties { todo!() }

    fn update_avg_pass_through_transparency(&mut self) { todo!() }
}
