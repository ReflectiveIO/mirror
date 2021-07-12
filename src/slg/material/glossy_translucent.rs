use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::material::{Material, MaterialType};
use crate::slg::textures::Texture;
use crate::slg::volume::Volume;

use super::material::MaterialTrait;
use crate::slg::bsdf::{BSDFEvent, BSDFEventType};

pub struct GlossyTranslucentMaterial {
    kd: Texture,
    kt: Texture,
    ks: Texture,
    ks_bf: Texture,
    nu: Texture,
    nu_bf: Texture,
    nv: Texture,
    nv_bf: Texture,
    ka: Texture,
    ka_bf: Texture,
    depth: Texture,
    depth_bf: Texture,
    index: Texture,
    index_bf: Texture,
    multi_bounce: bool,
    multi_bounce_bf: bool,
}

impl GlossyTranslucentMaterial {
    pub fn new(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
        kd: &Texture,
        kt: &Texture,
        ks: &Texture,
        ks2: &Texture,
        u: &Texture,
        u2: &Texture,
        v: &Texture,
        v2: &Texture,
        ka: &Texture,
        ka2: &Texture,
        d: &Texture,
        d2: &Texture,
        i: &Texture,
        i2: &Texture,
        multi_bounce: bool,
        multi_bounce2: bool,
    ) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_kd(&self) -> &Texture {
        &self.kd
    }
    pub fn get_kt(&self) -> &Texture {
        &self.kt
    }
    pub fn get_ks(&self) -> &Texture {
        &self.ks
    }
    pub fn get_ks_bf(&self) -> &Texture {
        &self.ks_bf
    }

    pub fn get_nu(&self) -> &Texture {
        &self.nu
    }
    pub fn get_nu_bf(&self) -> &Texture {
        &self.nu_bf
    }

    pub fn get_nv(&self) -> &Texture {
        &self.nv
    }

    pub fn get_nv_bf(&self) -> &Texture {
        &self.nv_bf
    }

    pub fn get_ka(&self) -> &Texture {
        &self.ka
    }

    pub fn get_ka_bf(&self) -> &Texture {
        &self.ka_bf
    }

    pub fn get_depth(&self) -> &Texture {
        &self.depth
    }

    pub fn get_depth_bf(&self) -> &Texture {
        &self.depth_bf
    }

    pub fn get_index(&self) -> &Texture {
        &self.index
    }

    pub fn get_index_bf(&self) -> &Texture {
        &self.index_bf
    }

    pub fn is_multi_bounce(&self) -> bool {
        self.multi_bounce
    }

    pub fn is_multi_bounce_bf(&self) -> bool {
        self.multi_bounce_bf
    }
}

impl MaterialTrait for GlossyTranslucentMaterial {
    fn get_type(&self) -> MaterialType {
        MaterialType::GlossyTranslucent
    }

    fn get_event_types(&self) -> BSDFEvent {
        BSDFEventType::GLOSSY | BSDFEventType::REFLECT | BSDFEventType::TRANSMIT
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
