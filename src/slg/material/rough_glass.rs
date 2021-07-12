use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::Texture;

use super::material::MaterialTrait;
use crate::slg::bsdf::{BSDFEvent, BSDFEventType};

pub struct RoughGlassMaterial {
    kr: Texture,
    kt: Texture,
    exterior_ior: Texture,
    interior_ior: Texture,
    nu: Texture,
    nv: Texture,
    film_thickness: Texture,
    film_ior: Texture,
}

impl RoughGlassMaterial {
    pub fn new(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
        refl: &Texture,
        trans: &Texture,
        exterior_ior_fact: &Texture,
        interior_ior_fact: &Texture,
        u: &Texture,
        v: &Texture,
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

    pub fn get_nu(&self) -> &Texture {
        &self.nu
    }
    pub fn get_nv(&self) -> &Texture {
        &self.nv
    }

    pub fn get_film_thickness(&self) -> &Texture {
        &self.film_thickness
    }

    pub fn get_film_ior(&self) -> &Texture {
        &self.film_ior
    }
}

impl MaterialTrait for RoughGlassMaterial {
    fn get_type(&self) -> MaterialType {
        MaterialType::Matte
    }

    fn get_event_types(&self) -> BSDFEvent {
        BSDFEventType::GLOSSY | BSDFEventType::REFLECT | BSDFEventType::TRANSMIT
    }

    fn albedo(&self, hit_point: &HitPoint) -> Spectrum {
        todo!()
    }

    fn evaluate_total(&self, hit_point: &HitPoint) -> Spectrum {
        todo!()
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

    fn add_referenced_textures(&mut self, v: &Vec<Texture>) {
        todo!()
    }

    fn update_texture_references(&mut self, old_tex: &Texture, new_tex: &Texture) {
        todo!()
    }

    fn to_properties(&self, imc: ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
