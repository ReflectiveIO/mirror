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
pub struct VelvetMaterial {
    kd: Texture,
    p1: Texture,
    p2: Texture,
    p3: Texture,
    thickness: Texture,
}

impl VelvetMaterial {
    pub fn new(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
        kd: &Texture,
        p1: &Texture,
        p2: &Texture,
        p3: &Texture,
        thickness: &Texture,
    ) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_kd(&self) -> &Texture { &self.kd }

    pub fn get_p1(&self) -> &Texture { &self.p1 }

    pub fn get_p2(&self) -> &Texture { &self.p2 }

    pub fn get_p3(&self) -> &Texture { &self.p3 }

    pub fn get_thickness(&self) -> &Texture { &self.thickness }
}

impl MaterialTrait for VelvetMaterial {
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

    fn add_referenced_textures(&mut self, v: &Vec<Texture>) { todo!() }

    fn update_texture_references(&mut self, old_tex: &Texture, new_tex: &Texture) { todo!() }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties { todo!() }
}

impl NamedObject for VelvetMaterial {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
