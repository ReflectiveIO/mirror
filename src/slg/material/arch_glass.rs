use super::material::MaterialTrait;
use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::object::NamedObject;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::bsdf::{BSDFEvent, BSDFEventType};
use crate::slg::material::MaterialType;
use crate::slg::textures::Texture;

pub struct ArchGlassMaterial {
    kr: Box<dyn Texture>,
    kt: Box<dyn Texture>,
    exterior_ior: Box<dyn Texture>,
    interior_ior: Box<dyn Texture>,
    cauchy_b: Box<dyn Texture>,
    film_thickness: Box<dyn Texture>,
    film_ior: Box<dyn Texture>,
}

impl ArchGlassMaterial {
    pub fn new(
        front_transp: &Box<dyn Texture>,
        back_transp: &Box<dyn Texture>,
        emitted: &Box<dyn Texture>,
        bump: &Box<dyn Texture>,
        refl: &Box<dyn Texture>,
        trans: &Box<dyn Texture>,
        exterior_fact: &Box<dyn Texture>,
        interior_fact: &Box<dyn Texture>,
        b: &Box<dyn Texture>,
        film_thickness: &Box<dyn Texture>,
        film_ior: &Box<dyn Texture>,
    ) -> Self {
        todo!()
    }

    pub fn get_kr(&self) -> &Box<dyn Texture> { &self.kr }

    pub fn get_kt(&self) -> &Box<dyn Texture> { &self.kt }

    pub fn get_exterior_ior(&self) -> &Box<dyn Texture> { &self.exterior_ior }

    pub fn get_interior_ior(&self) -> &Box<dyn Texture> { &self.interior_ior }

    pub fn get_cauchy_b(&self) -> &Box<dyn Texture> { &self.cauchy_b }

    pub fn get_film_thickness(&self) -> &Box<dyn Texture> { &self.film_thickness }

    pub fn get_film_ior(&self) -> &Box<dyn Texture> { &self.film_ior }

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
        Spectrum::new()
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
        Spectrum::new()
    }
}

impl MaterialTrait for ArchGlassMaterial {
    fn get_type(&self) -> MaterialType { MaterialType::ArchGlass }

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

impl NamedObject for ArchGlassMaterial {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
