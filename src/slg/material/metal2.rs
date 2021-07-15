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

#[derive(Default)]
pub struct Metal2Material {
    fresnel_tex: Texture,
    n: Texture,
    k: Texture,
    nu: Texture,
    nv: Texture,
}

impl Metal2Material {
    pub fn new(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
        nn: &Texture,
        kk: &Texture,
        u: &Texture,
        v: &Texture,
    ) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn fresnel(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
        fresnel_texture: &Texture,
        u: &Texture,
        v: &Texture,
    ) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_fresnel(&self) -> &Texture { &self.fresnel_tex }

    pub fn get_n(&self) -> &Texture { &self.n }

    pub fn get_k(&self) -> &Texture { &self.k }

    pub fn get_nu(&self) -> &Texture { &self.nu }

    pub fn get_nv(&self) -> &Texture { &self.nv }
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

    fn add_referenced_textures(&mut self, v: &Vec<Texture>) { todo!() }

    fn update_texture_references(&mut self, old_tex: &Texture, new_tex: &Texture) { todo!() }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties { todo!() }
}

impl NamedObject for Metal2Material {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
