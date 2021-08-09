use crate::rays::color::Spectrum;
use crate::rays::geometry::{Ray, Vector};
use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::bsdf::{BSDFEvent, HitPoint};
use crate::slg::image_map::ImageMapCache;
use crate::slg::material::{BaseMaterial, Material, MaterialType};
use crate::slg::textures::Texture;

pub struct ClearVolume {
    base: BaseMaterial,
    sigma_a: Box<dyn Texture>,
}

impl ClearVolume {
    pub fn new(
        ior_tex: &Box<dyn Texture>,
        emi_tex: &Box<dyn Texture>,
        a: &Box<dyn Texture>,
    ) -> Self {
        todo!()
    }

    pub fn scatter(
        &self,
        ray: &Ray,
        u: f32,
        scattered_start: bool,
        connection_throughput: &Spectrum,
        connection_emission: &Spectrum,
    ) -> f32 {
        todo!()
    }

    pub fn get_sigma_a(&self) -> &Box<dyn Texture> { &self.sigma_a }
}

impl Material for ClearVolume {
    fn base(&self) -> &BaseMaterial { &self.base }

    fn get_type(&self) -> MaterialType { todo!() }

    fn get_event_types(&self) -> BSDFEvent { todo!() }

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
