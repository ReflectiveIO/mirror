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

pub struct VelvetMaterial {
    base: BaseMaterial,

    kd: Box<dyn Texture>,
    p1: Box<dyn Texture>,
    p2: Box<dyn Texture>,
    p3: Box<dyn Texture>,
    thickness: Box<dyn Texture>,
}

impl VelvetMaterial {
    pub fn new(
        front_transp: &Box<dyn Texture>,
        back_transp: &Box<dyn Texture>,
        emitted: &Box<dyn Texture>,
        bump: &Box<dyn Texture>,
        kd: &Box<dyn Texture>,
        p1: &Box<dyn Texture>,
        p2: &Box<dyn Texture>,
        p3: &Box<dyn Texture>,
        thickness: &Box<dyn Texture>,
    ) -> Self {
        todo!()
    }

    pub fn get_kd(&self) -> &Box<dyn Texture> { &self.kd }

    pub fn get_p1(&self) -> &Box<dyn Texture> { &self.p1 }

    pub fn get_p2(&self) -> &Box<dyn Texture> { &self.p2 }

    pub fn get_p3(&self) -> &Box<dyn Texture> { &self.p3 }

    pub fn get_thickness(&self) -> &Box<dyn Texture> { &self.thickness }
}

impl Material for VelvetMaterial {
    fn base(&self) -> &BaseMaterial { &self.base }

    fn get_type(&self) -> MaterialType { MaterialType::Velvet }

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
