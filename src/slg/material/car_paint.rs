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

pub struct CarPaintMaterial {
    pub kd: Box<dyn Texture>,

    pub ks1: Box<dyn Texture>,
    pub ks2: Box<dyn Texture>,
    pub ks3: Box<dyn Texture>,

    pub m1: Box<dyn Texture>,
    pub m2: Box<dyn Texture>,
    pub m3: Box<dyn Texture>,

    pub r1: Box<dyn Texture>,
    pub r2: Box<dyn Texture>,
    pub r3: Box<dyn Texture>,
    pub ka: Box<dyn Texture>,
    pub depth: Box<dyn Texture>,
}

impl CarPaintMaterial {
    pub fn new(
        front_transp: &Box<dyn Texture>,
        back_transp: &Box<dyn Texture>,
        emitted: &Box<dyn Texture>,
        bump: &Box<dyn Texture>,
        kd: &Box<dyn Texture>,
        ks1: &Box<dyn Texture>,
        ks2: &Box<dyn Texture>,
        ks3: &Box<dyn Texture>,
        m1: &Box<dyn Texture>,
        m2: &Box<dyn Texture>,
        m3: &Box<dyn Texture>,
        r1: &Box<dyn Texture>,
        r2: &Box<dyn Texture>,
        r3: &Box<dyn Texture>,
        ka: &Box<dyn Texture>,
        depth: &Box<dyn Texture>,
    ) -> Self {
        todo!()
    }

    pub fn nb_presets() -> i8 { 8 }
}

impl MaterialTrait for CarPaintMaterial {
    fn get_type(&self) -> MaterialType { MaterialType::CarPaint }

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

impl NamedObject for CarPaintMaterial {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
