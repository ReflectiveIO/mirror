use crate::rays::color::Spectrum;
use crate::rays::geometry::{Ray, Vector};
use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::bsdf::{BSDFEvent, HitPoint};
use crate::slg::image_map::ImageMapCache;
use crate::slg::material::{MaterialTrait, MaterialType};
use crate::slg::textures::Texture;
use crate::slg::volume::SchlickScatter;

pub struct HeterogeneousVolume {
    sigma_a: Box<dyn Texture>,
    sigma_s: Box<dyn Texture>,
    schlick_scatter: SchlickScatter,
    step_size: f32,
    max_steps_count: usize,
    multi_scattering: bool,
}

impl HeterogeneousVolume {
    pub fn new(
        ior_tex: &Box<dyn Texture>,
        emi_tex: &Box<dyn Texture>,
        a: &Box<dyn Texture>,
        s: &Box<dyn Texture>,
        g: &Box<dyn Texture>,
        step_size: f32,
        max_steps_count: usize,
        multi_scattering: bool,
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

    pub fn get_sigma_s(&self) -> &Box<dyn Texture> { &self.sigma_s }

    pub fn get_g(&self) -> &Box<dyn Texture> { &self.schlick_scatter.g }

    pub fn get_step_size(&self) -> f32 { self.step_size }

    pub fn get_max_steps_count(&self) -> usize { self.max_steps_count }

    pub fn is_multi_scattering(&self) -> bool { self.multi_scattering }
}

impl NamedObject for HeterogeneousVolume {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}

impl MaterialTrait for HeterogeneousVolume {
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
