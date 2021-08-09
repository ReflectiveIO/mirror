use crate::rays::color::Spectrum;
use crate::rays::geometry::{Ray, Vector};
use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::bsdf::{BSDFEvent, HitPoint};
use crate::slg::image_map::ImageMapCache;
use crate::slg::material::{MaterialTrait, MaterialType};
use crate::slg::textures::Texture;

#[derive(Clone, PartialEq)]
pub struct Volume {
    ior_texture: Box<dyn Texture>,
    volume_emission_texture: Box<dyn Texture>,
    volume_light_id: usize,
    priority: i8,
}

impl Volume {
    pub fn new(ior: &Box<dyn Texture>, emission: &Box<dyn Texture>) -> Self { todo!() }

    pub fn set_volume_light_id(&mut self, id: usize) { self.volume_light_id = id }

    pub fn get_volume_light_id(&self) -> usize { self.volume_light_id }

    pub fn set_priority(&mut self, p: i8) { self.priority = p; }

    pub const fn get_priority(&self) -> i8 { self.priority }

    pub const fn get_ior_texture(&self) -> &Box<dyn Texture> { &self.ior_texture }

    pub const fn get_volume_emission_texture(&self) -> &Box<dyn Texture> {
        &self.volume_emission_texture
    }

    pub fn get_ior(&self, hp: &HitPoint) -> f32 { self.ior_texture.get_float_value(hp) }

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
}

impl NamedObject for Volume {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}

impl MaterialTrait for Volume {
    fn get_type(&self) -> MaterialType { todo!() }

    fn get_event_types(&self) -> BSDFEvent { todo!() }

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

/// An utility class
pub struct SchlickScatter {
    pub volume: Volume,
    pub g: Box<dyn Texture>,
}

impl SchlickScatter {
    pub fn new(volume: &Volume, g: &Box<dyn Texture>) -> Self { todo!() }

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
}
