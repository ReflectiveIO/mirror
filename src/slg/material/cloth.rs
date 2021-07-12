use crate::rays::color::Spectrum;
use crate::rays::geometry::{Point, Vector, UV};
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::material::material::MaterialTrait;
use crate::slg::textures::Texture;

pub struct ClothMaterial {
    preset: ClothPreset,
    weft_kd: Texture,
    weft_ks: Texture,
    warp_kd: Texture,
    warp_ks: Texture,
    repeat_u: f32,
    repeat_v: f32,
    specular_normalization: f32,
}

impl ClothMaterial {
    pub fn new(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
        preset: ClothPreset,
        weft_kd: &Texture,
        weft_ks: &Texture,
        warp_kd: &Texture,
        warp_ks: &Texture,
        repeat_u: f32,
        repeat_v: f32,
    ) -> Self {
        Self {
            preset,
            weft_kd: Default::default(),
            weft_ks: Default::default(),
            warp_kd: Default::default(),
            warp_ks: Default::default(),
            repeat_u,
            repeat_v,
            specular_normalization: 0.0,
        }
    }

    pub fn get_preset(&self) -> &ClothPreset {
        &self.preset
    }

    pub fn get_weft_kd(&self) -> &Texture {
        &self.weft_kd
    }

    pub fn get_weft_ks(&self) -> &Texture {
        &self.weft_ks
    }

    pub fn get_warp_kd(&self) -> &Texture {
        &self.warp_kd
    }

    pub fn get_warp_ks(&self) -> &Texture {
        &self.warp_ks
    }

    pub fn get_repeat_u(&self) -> f32 {
        self.repeat_u
    }

    pub fn get_repeat_v(&self) -> f32 {
        self.repeat_v
    }

    pub fn get_specular_normalization(&self) -> f32 {
        self.specular_normalization
    }

    fn set_preset(&mut self) {}
    fn get_yarn(&self, u_i: f32, v_i: f32, uv: &UV, umax: f32, scale: f32) -> Yarn {}
    fn get_yarn_uv(&self, yarn: &Yarn, center: &Point, xy: &Point, uv: &UV, umax_mod: f32) {}

    fn radius_of_curvature(&self, yarn: &Yarn, u: f32, umax_mod: f32) -> f32 {}
    fn eval_specular(&self, yarn: &Yarn, uv: &UV, umax: f32, wo: &Vector, vi: &Vector) -> f32 {}
    fn eval_integrand(
        &self,
        yarn: &Yarn,
        uv: &UV,
        umax_mod: f32,
        om_i: &Vector,
        om_r: &Vector,
    ) -> f32 {
        0.0
    }

    fn eval_filament_integrand(
        &self,
        yarn: &Yarn,
        om_i: &Vector,
        om_r: &Vector,
        u: f32,
        v: f32,
        umax_mod: f32,
    ) -> f32 {
        0.0
    }

    fn eval_staple_integrand(
        &self,
        yarn: &Yarn,
        om_i: &Vector,
        om_r: &Vector,
        u: f32,
        v: f32,
        umax_mod: f32,
    ) -> f32 {
        0.0
    }
}

impl MaterialTrait for ClothMaterial {
    fn get_type(&self) -> MaterialType {
        MaterialType::Cloth
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
