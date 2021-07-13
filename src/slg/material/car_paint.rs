use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::bsdf::{BSDFEvent, BSDFEventType};
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::Texture;

use super::material::MaterialTrait;
use crate::slg::material::MaterialType;

#[derive(Default)]
pub struct CarPaintMaterial {
    pub kd: Texture,

    pub ks1: Texture,
    pub ks2: Texture,
    pub ks3: Texture,

    pub m1: Texture,
    pub m2: Texture,
    pub m3: Texture,

    pub r1: Texture,
    pub r2: Texture,
    pub r3: Texture,
    pub ka: Texture,
    pub depth: Texture,
}

impl CarPaintMaterial {
    pub fn new(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
        kd: &Texture,
        ks1: &Texture,
        ks2: &Texture,
        ks3: &Texture,

        m1: &Texture,
        m2: &Texture,
        m3: &Texture,

        r1: &Texture,
        r2: &Texture,
        r3: &Texture,
        ka: &Texture,
        depth: &Texture,
    ) -> Self {
        Self {
            kd: Texture::default(),
            ks1: Default::default(),
            ks2: Default::default(),
            ks3: Default::default(),
            m1: Default::default(),
            m2: Default::default(),
            m3: Default::default(),
            r1: Default::default(),
            r2: Default::default(),
            r3: Default::default(),
            ka: Default::default(),
            depth: Default::default(),
        }
    }

    pub fn nb_presets() -> i8 {
        8
    }
}

impl MaterialTrait for CarPaintMaterial {
    fn get_type(&self) -> MaterialType {
        MaterialType::CarPaint
    }

    fn get_event_types(&self) -> BSDFEvent {
        BSDFEventType::GLOSSY | BSDFEventType::REFLECT
    }

    fn albedo(&self, hit_point: &HitPoint) -> Spectrum {
        todo!()
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

    fn to_properties(&self, imc: ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
