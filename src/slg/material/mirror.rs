use super::material::MaterialTrait;
use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::object::NamedObject;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::bsdf::{BSDFEvent, BSDFEventType};
use crate::slg::material::MaterialType;
use crate::slg::textures::Texture;

pub struct MirrorMaterial {
    kr: Box<dyn Texture>,
}

impl MirrorMaterial {
    pub fn new(
        front_transp: &Box<dyn Texture>,
        back_transp: &Box<dyn Texture>,
        emitted: &Box<dyn Texture>,
        bump: &Box<dyn Texture>,
        refl: &Box<dyn Texture>,
    ) -> Self {
        todo!()
    }

    pub fn get_kr(&self) -> &Box<dyn Texture> { &self.kr }
}

impl MaterialTrait for MirrorMaterial {
    fn get_type(&self) -> MaterialType { MaterialType::Mirror }

    fn get_event_types(&self) -> BSDFEvent { BSDFEventType::SPECULAR | BSDFEventType::REFLECT }

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
}

impl NamedObject for MirrorMaterial {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
