use super::material::{Material, MaterialTrait};
use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::bsdf::BSDFEvent;
use crate::slg::image_map::ImageMapCache;
use crate::slg::material::MaterialType;
use crate::slg::textures::Texture;
use crate::slg::volume::Volume;

pub struct MixMaterial {
    a: Box<dyn MaterialTrait>,
    b: Box<dyn MaterialTrait>,
    mix_factor: Box<dyn Texture>,
    event_types: BSDFEvent,
    is_light_source: bool,
    is_delta: bool,
}

impl MixMaterial {
    pub fn new(
        front_transp: &Box<dyn Texture>,
        back_transp: &Box<dyn Texture>,
        emitted: &Box<dyn Texture>,
        bump: &Box<dyn Texture>,
        a: Box<dyn MaterialTrait>,
        b: Box<dyn MaterialTrait>,
        mix: Box<dyn Texture>,
    ) -> Self {
        Self {
            a,
            b,
            mix_factor: mix,

            event_types: Default::default(),
            is_light_source: false,
            is_delta: false,
        }
    }

    pub fn get_a(&self) -> &Box<dyn MaterialTrait> { &self.a }

    pub fn get_b(&self) -> &Box<dyn MaterialTrait> { &self.b }

    pub fn get_mix_factor(&self) -> &Box<dyn Texture> { &self.mix_factor }
}

impl MaterialTrait for MixMaterial {
    fn get_type(&self) -> MaterialType { MaterialType::Mix }

    fn get_event_types(&self) -> BSDFEvent { self.event_types }

    fn is_light_source(&self) -> bool { todo!() }

    fn is_delta(&self) -> bool { todo!() }

    fn get_pass_through_transparency(
        &self,
        hit_point: &HitPoint,
        local_fixed_dir: Vector,
        pass_through_event: f32,
        back_tracing: bool,
    ) -> Spectrum {
        todo!()
    }

    fn get_emitted_radiance(&self, hit_point: &HitPoint, one_over_primitive_area: f32) -> Spectrum {
        todo!()
    }

    fn get_emitted_radiance_y(&self, one_over_primitive_area: f32) -> f32 { todo!() }

    fn get_interior_volume(&self, hit_point: &HitPoint, pass_through_event: f32) -> &Volume {
        todo!()
    }

    fn get_exterior_volume(&self, hit_point: &HitPoint, pass_through_event: f32) -> &Volume {
        todo!()
    }

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

    fn update_material_references(
        &mut self,
        old_mat: &Box<dyn MaterialTrait>,
        new_mat: &Box<dyn MaterialTrait>,
    ) {
        todo!()
    }

    fn is_referencing(&self, mat: &Box<dyn MaterialTrait>) -> bool { todo!() }

    fn add_referenced_materials(&mut self, v: &Vec<Box<dyn MaterialTrait>>) { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) { todo!() }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        todo!()
    }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties { todo!() }

    fn update_avg_pass_through_transparency(&mut self) { todo!() }
}

impl NamedObject for MixMaterial {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
