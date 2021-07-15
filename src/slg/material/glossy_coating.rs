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
use crate::slg::volume::Volume;

pub struct GlossyCoatingMaterial {
    mat_base: Box<dyn MaterialTrait>,
    ks: Texture,
    nu: Texture,
    nv: Texture,
    ka: Texture,
    depth: Texture,
    index: Texture,
    multi_bounce: bool,
}

impl GlossyCoatingMaterial {
    pub fn new(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
        mb: Box<dyn MaterialTrait>,
        ks: Texture,
        u: Texture,
        v: Texture,
        ka: Texture,
        d: Texture,
        i: Texture,
        multi_bounce: bool,
    ) -> Self {
        Self {
            mat_base: mb,
            ks,
            nu: u,
            nv: v,
            ka,
            depth: d,
            index: i,
            multi_bounce,
        }
    }

    pub fn get_material_base(&self) -> &Box<dyn MaterialTrait> { &self.mat_base }

    pub fn get_ks(&self) -> &Texture { &self.ks }

    pub fn get_nu(&self) -> &Texture { &self.nu }

    pub fn get_nv(&self) -> &Texture { &self.nv }

    pub fn get_ka(&self) -> &Texture { &self.ka }

    pub fn get_depth(&self) -> &Texture { &self.depth }

    pub fn get_index(&self) -> &Texture { &self.index }

    pub fn is_multi_bounce(&self) -> bool { self.multi_bounce }
}

impl MaterialTrait for GlossyCoatingMaterial {
    fn get_type(&self) -> MaterialType { MaterialType::GlossyCoating }

    fn get_event_types(&self) -> BSDFEvent {
        BSDFEventType::GLOSSY | BSDFEventType::REFLECT | self.mat_base.get_event_types()
    }

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

    fn add_referenced_textures(&mut self, v: &Vec<Texture>) { todo!() }

    fn update_texture_references(&mut self, old_tex: &Texture, new_tex: &Texture) { todo!() }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties { todo!() }

    fn update_avg_pass_through_transparency(&mut self) { todo!() }
}

impl NamedObject for GlossyCoatingMaterial {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
