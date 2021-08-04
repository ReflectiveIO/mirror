use super::material::MaterialTrait;
use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::bsdf::{BSDFEvent, BSDFEventType};
use crate::slg::image_map::ImageMapCache;
use crate::slg::material::MaterialType;
use crate::slg::textures::Texture;

pub struct Metal2Material {
    fresnel_tex: Box<dyn Texture>,
    n: Box<dyn Texture>,
    k: Box<dyn Texture>,
    nu: Box<dyn Texture>,
    nv: Box<dyn Texture>,
}

impl Metal2Material {
    pub fn new(
        front_transp: &Box<dyn Texture>,
        back_transp: &Box<dyn Texture>,
        emitted: &Box<dyn Texture>,
        bump: &Box<dyn Texture>,
        nn: &Box<dyn Texture>,
        kk: &Box<dyn Texture>,
        u: &Box<dyn Texture>,
        v: &Box<dyn Texture>,
    ) -> Self {
        todo!()
    }

    pub fn fresnel(
        front_transp: &Box<dyn Texture>,
        back_transp: &Box<dyn Texture>,
        emitted: &Box<dyn Texture>,
        bump: &Box<dyn Texture>,
        fresnel_texture: &Box<dyn Texture>,
        u: &Box<dyn Texture>,
        v: &Box<dyn Texture>,
    ) -> Self {
        todo!()
    }

    pub fn get_fresnel(&self) -> &Box<dyn Texture> { &self.fresnel_tex }

    pub fn get_n(&self) -> &Box<dyn Texture> { &self.n }

    pub fn get_k(&self) -> &Box<dyn Texture> { &self.k }

    pub fn get_nu(&self) -> &Box<dyn Texture> { &self.nu }

    pub fn get_nv(&self) -> &Box<dyn Texture> { &self.nv }
}

impl MaterialTrait for Metal2Material {
    fn get_type(&self) -> MaterialType { MaterialType::Metal2 }

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

impl NamedObject for Metal2Material {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
