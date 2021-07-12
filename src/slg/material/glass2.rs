use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::Texture;

use super::material::MaterialTrait;

pub struct Glass2Material {
    kd: Texture,
    ks: Texture,
    nu: Texture,
    nv: Texture,
    ka: Texture,
    depth: Texture,
    index: Texture,
    multi_bounce: bool,
}

impl Glass2Material {
    pub fn new(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
        kd: &Texture,
        ks: &Texture,
        u: &Texture,
        v: &Texture,
        ka: &Texture,
        d: &Texture,
        i: &Texture,
        multi_bounce: bool,
    ) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_kd(&self) -> &Texture {
        &self.kd
    }

    pub fn get_ks(&self) -> &Texture {
        &self.ks
    }

    pub fn get_nu(&self) -> &Texture {
        &self.nu
    }

    pub fn get_nv(&self) -> &Texture {
        &self.nv
    }

    pub fn get_ka(&self) -> &Texture {
        &self.ka
    }

    pub fn get_depth(&self) -> &Texture {
        &self.depth
    }

    pub fn get_index(&self) -> &Texture {
        &self.index
    }

    pub fn is_multi_bounce(&self) -> bool {
        self.multi_bounce
    }
}

impl MaterialTrait for Glass2Material {
    fn get_type(&self) -> MaterialType {
        MaterialType::Glass2
    }

    fn get_event_types(&self) -> BSDFEvent {
        todo!()
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
