use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::textures::Texture;

use super::material::MaterialTrait;
use crate::slg::bsdf::{BSDFEvent, BSDFEventType};
use crate::slg::material::MaterialType;

pub struct GlassMaterial {
    kr: Texture,
    kt: Texture,
    exterior_ior: Texture,
    interior_ior: Texture,
    cauchy_b: Texture,
    film_thickness: Texture,
    film_ior: Texture,
}

impl GlassMaterial {
    pub fn new(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
        refl: &Texture,
        trans: &Texture,
        exterior_fact: &Texture,
        interior_fact: &Texture,
        b: &Texture,
        film_thickness: &Texture,
        film_ior: &Texture,
    ) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_kr(&self) -> &Texture {
        &self.kr
    }

    pub fn get_kt(&self) -> &Texture {
        &self.kt
    }

    pub fn get_exterior_ior(&self) -> &Texture {
        &self.exterior_ior
    }

    pub fn get_interior_ior(&self) -> &Texture {
        &self.interior_ior
    }

    pub fn get_cauchy_b(&self) -> &Texture {
        &self.cauchy_b
    }

    pub fn get_film_thickness(&self) -> &Texture {
        &self.film_thickness
    }

    pub fn get_film_ior(&self) -> &Texture {
        &self.film_ior
    }

    pub fn eval_specular_reflection(
        hp: &HitPoint,
        local_fixed_dir: &Vector,
        kr: &Spectrum,
        nc: f32,
        nt: f32,
        local_sampled_dir: &Vector,
        local_film_thickness: f32,
        local_film_ior: f32,
    ) -> Spectrum {
        Spectrum::new(())
    }

    pub fn eval_specular_transmission(
        hp: &HitPoint,
        local_fixed_dir: Vector,
        u0: f32,
        kt: &Spectrum,
        nc: f32,
        nt: f32,
        cauchy_b: f32,
        local_sampled_dir: Vector,
    ) -> Spectrum {
        Spectrum::new(())
    }
}

impl MaterialTrait for GlassMaterial {
    fn get_type(&self) -> MaterialType {
        MaterialType::Glass
    }

    fn get_event_types(&self) -> BSDFEvent {
        BSDFEventType::SPECULAR | BSDFEventType::REFLECT | BSDFEventType::TRANSMIT
    }

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
}
