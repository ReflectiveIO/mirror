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

pub struct Glossy2Material {
    base: BaseMaterial,

    kd: Box<dyn Texture>,
    ks: Box<dyn Texture>,
    nu: Box<dyn Texture>,
    nv: Box<dyn Texture>,
    ka: Box<dyn Texture>,
    depth: Box<dyn Texture>,
    index: Box<dyn Texture>,
    multi_bounce: bool,
}

impl Glossy2Material {
    pub fn new(
        front_transp: &Box<dyn Texture>,
        back_transp: &Box<dyn Texture>,
        emitted: &Box<dyn Texture>,
        bump: &Box<dyn Texture>,
        kd: &Box<dyn Texture>,
        ks: &Box<dyn Texture>,
        u: &Box<dyn Texture>,
        v: &Box<dyn Texture>,
        ka: &Box<dyn Texture>,
        d: &Box<dyn Texture>,
        i: &Box<dyn Texture>,
        multi_bounce: bool,
    ) -> Self {
        todo!()
    }

    pub fn get_kd(&self) -> &Box<dyn Texture> { &self.kd }

    pub fn get_ks(&self) -> &Box<dyn Texture> { &self.ks }

    pub fn get_nu(&self) -> &Box<dyn Texture> { &self.nu }

    pub fn get_nv(&self) -> &Box<dyn Texture> { &self.nv }

    pub fn get_ka(&self) -> &Box<dyn Texture> { &self.ka }

    pub fn get_depth(&self) -> &Box<dyn Texture> { &self.depth }

    pub fn get_index(&self) -> &Box<dyn Texture> { &self.index }

    pub fn is_multi_bounce(&self) -> bool { self.multi_bounce }
}

impl Material for Glossy2Material {
    fn base(&self) -> &BaseMaterial { &self.base }

    fn get_type(&self) -> MaterialType { MaterialType::Glossy2 }

    fn get_event_types(&self) -> BSDFEvent { BSDFEventType::GLOSSY | BSDFEventType::REFLECT }

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
