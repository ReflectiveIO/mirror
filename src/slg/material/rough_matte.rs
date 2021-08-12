use super::material::Material;
use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::Properties;
use crate::slg::bsdf::{BSDFEvent, BSDFEventType, HitPoint};
use crate::slg::image_map::ImageMapCache;
use crate::slg::material::base::BaseMaterial;
use crate::slg::material::MaterialType;
use crate::slg::textures::Texture;

pub struct RoughMatteMaterial {
    base: BaseMaterial,

    kd: Box<dyn Texture>,
    sigma: Box<dyn Texture>,
}

impl RoughMatteMaterial {
    pub fn new(
        front_transp: &Box<dyn Texture>,
        back_transp: &Box<dyn Texture>,
        emitted: &Box<dyn Texture>,
        bump: &Box<dyn Texture>,
        col: &Box<dyn Texture>,
        s: &Box<dyn Texture>,
    ) -> Self {
        todo!()
    }

    pub fn get_kd(&self) -> &Box<dyn Texture> { &self.kd }

    pub fn get_sigma(&self) -> &Box<dyn Texture> { &self.sigma }
}

impl Material for RoughMatteMaterial {
    fn base(&self) -> &BaseMaterial { &self.base }

    fn base_mut(&mut self) -> &mut BaseMaterial { &mut self.base }

    fn get_type(&self) -> MaterialType { MaterialType::RoughMatte }

    fn get_event_types(&self) -> BSDFEvent { BSDFEventType::DIFFUSE | BSDFEventType::REFLECT }

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
