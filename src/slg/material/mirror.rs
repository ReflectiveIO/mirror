use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::textures::Texture;

use super::material::MaterialTrait;
use crate::slg::bsdf::{BSDFEvent, BSDFEventType};
use crate::slg::material::MaterialType;

#[derive(Default)]
pub struct MirrorMaterial {
    kr: Texture,
}

impl MirrorMaterial {
    pub fn new(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
        refl: &Texture,
    ) -> Self {
        MirrorMaterial {
            kr: Texture::default(),
        }
    }

    pub fn get_kr(&self) -> &Texture {
        &self.kr
    }
}

impl MaterialTrait for MirrorMaterial {
    fn get_type(&self) -> MaterialType {
        MaterialType::Mirror
    }

    fn get_event_types(&self) -> BSDFEvent {
        BSDFEventType::SPECULAR | BSDFEventType::REFLECT
    }

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

    fn add_referenced_textures(&mut self, v: &Vec<Texture>) {
        todo!()
    }

    fn update_texture_references(&mut self, old_tex: &Texture, new_tex: &Texture) {
        todo!()
    }
}
