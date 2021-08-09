use super::material::Material;
use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::bsdf::{BSDFEvent, BSDFEventType, HitPoint};
use crate::slg::image_map::ImageMapCache;
use crate::slg::material::base::BaseMaterial;
use crate::slg::material::MaterialType;
use crate::slg::textures::Texture;

pub struct RoughGlassMaterial {
    base: BaseMaterial,

    kr: Box<dyn Texture>,
    kt: Box<dyn Texture>,
    exterior_ior: Box<dyn Texture>,
    interior_ior: Box<dyn Texture>,
    nu: Box<dyn Texture>,
    nv: Box<dyn Texture>,
    film_thickness: Box<dyn Texture>,
    film_ior: Box<dyn Texture>,
}

impl RoughGlassMaterial {
    pub fn new(
        front_transp: &Box<dyn Texture>,
        back_transp: &Box<dyn Texture>,
        emitted: &Box<dyn Texture>,
        bump: &Box<dyn Texture>,
        refl: &Box<dyn Texture>,
        trans: &Box<dyn Texture>,
        exterior_ior_fact: &Box<dyn Texture>,
        interior_ior_fact: &Box<dyn Texture>,
        u: &Box<dyn Texture>,
        v: &Box<dyn Texture>,
        film_thickness: &Box<dyn Texture>,
        film_ior: &Box<dyn Texture>,
    ) -> Self {
        todo!()
    }

    pub fn get_kr(&self) -> &Box<dyn Texture> { &self.kr }

    pub fn get_kt(&self) -> &Box<dyn Texture> { &self.kt }

    pub fn get_exterior_ior(&self) -> &Box<dyn Texture> { &self.exterior_ior }

    pub fn get_interior_ior(&self) -> &Box<dyn Texture> { &self.interior_ior }

    pub fn get_nu(&self) -> &Box<dyn Texture> { &self.nu }

    pub fn get_nv(&self) -> &Box<dyn Texture> { &self.nv }

    pub fn get_film_thickness(&self) -> &Box<dyn Texture> { &self.film_thickness }

    pub fn get_film_ior(&self) -> &Box<dyn Texture> { &self.film_ior }
}

impl Material for RoughGlassMaterial {
    fn base(&self) -> &BaseMaterial { &self.base }

    fn get_type(&self) -> MaterialType { MaterialType::RoughGlass }

    fn get_event_types(&self) -> BSDFEvent {
        BSDFEventType::GLOSSY | BSDFEventType::REFLECT | BSDFEventType::TRANSMIT
    }

    fn albedo(&self, hit_point: &HitPoint) -> Spectrum { todo!() }

    fn evaluate_total(&self, hit_point: &HitPoint) -> Spectrum { todo!() }

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

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) { todo!() }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        todo!()
    }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties { todo!() }
}
