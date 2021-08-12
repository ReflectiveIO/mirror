use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::Properties;
use crate::slg::bsdf::{BSDFEvent, HitPoint};
use crate::slg::core::SampleableSphericalFunction;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::material::{Material, MaterialEmissionDLSType, MaterialType};
use crate::slg::textures::Texture;
use crate::slg::volume::Volume;

#[derive(Clone, PartialEq)]
pub struct BaseMaterial {
    mat_id: u32,
    light_id: u32,

    direct_light_sampling_type: MaterialEmissionDLSType,

    emitted_importance: f32,
    emitted_gain: Spectrum,
    emitted_factor: Spectrum,
    emitted_power: f32,
    emitted_efficiency: f32,
    emitted_theta: f32,
    emitted_cos_theta_max: f32,

    emitted_power_normalize: bool,
    emitted_gain_normalize: bool,
    emitted_temperature: f32,
    emitted_normalize_temperature: bool,

    front_transparency_tex: Box<dyn Texture>,
    back_transparency_tex: Box<dyn Texture>,
    pass_through_shadow_transparency: Spectrum,
    emitted_tex: Option<Box<dyn Texture>>,
    bump_tax: Option<Box<dyn Texture>>,
    bump_sample_distance: f32,

    emission_map: ImageMap,
    emission_func: SampleableSphericalFunction,
    interior_volume: Box<Volume>,
    exterior_volume: Box<Volume>,

    glossiness: f32,
    avg_pass_through_transparency: f32,

    is_visible_indirect_diffuse: bool,
    is_visible_indirect_glossy: bool,
    is_visible_indirect_specular: bool,

    use_primitive_area: bool,
    is_shadow_catcher: bool,
    is_shadow_cater_only_infinite_lights: bool,
    is_photon_gi_enabled: bool,
    is_holdout: bool,
}

impl Default for BaseMaterial {
    fn default() -> Self { todo!() }
}

impl BaseMaterial {
    pub fn new(
        front_transp: &Box<dyn Texture>,
        back_transp: &Box<dyn Texture>,
        emitted: &Box<dyn Texture>,
        bump: &Box<dyn Texture>,
    ) -> Self {
        todo!()
    }

    fn update_emitted_factor(&mut self) { todo!() }

    fn update_avg_pass_through_transparency(&mut self) { todo!() }
}

impl Material for BaseMaterial {
    fn base(&self) -> &BaseMaterial { unimplemented!() }

    fn base_mut(&mut self) -> &mut BaseMaterial { unimplemented!() }

    fn set_light_id(&mut self, id: u32) { self.light_id = id }

    fn get_light_id(&self) -> u32 { self.light_id }

    fn set_id(&mut self, id: u32) { self.mat_id = id }

    fn get_id(&self) -> u32 { self.mat_id }

    fn set_emitted_gain(&mut self, v: Spectrum) {
        self.emitted_gain = v;
        self.update_emitted_factor();
    }

    fn get_emitted_gain(&self) -> &Spectrum { &self.emitted_gain }

    fn set_emitted_power(&mut self, v: f32) {
        self.emitted_power = v;
        self.update_emitted_factor();
    }

    fn get_emitted_power(&self) -> f32 { self.emitted_power }

    fn set_emitted_power_normalize(&mut self, v: bool) { self.emitted_power_normalize = v }

    fn is_emitted_power_normalize(&self) -> bool { self.emitted_power_normalize }

    fn set_emitted_gain_normalize(&mut self, v: bool) { self.emitted_gain_normalize = v }

    fn is_emitted_gain_normalize(&self) -> bool { self.emitted_gain_normalize }

    fn set_emitted_efficiency(&mut self, v: f32) {
        self.emitted_efficiency = v;
        self.update_emitted_factor();
    }

    fn get_emitted_efficiency(&self) -> f32 { self.emitted_efficiency }

    fn get_emitted_factor(&self) -> &Spectrum { &self.emitted_factor }

    fn set_emitted_theta(&mut self, theta: f32) { self.emitted_theta = theta }

    fn get_emitted_theta(&self) -> f32 { self.emitted_theta }

    fn get_emitted_cos_theta_max(&self) -> f32 { self.emitted_cos_theta_max }

    fn set_emitted_temperature(&mut self, v: f32) {
        self.emitted_temperature = v;
        self.update_emitted_factor();
    }

    fn set_emitted_temperature_normalize(&mut self, v: bool) {
        self.emitted_normalize_temperature = v;
        self.update_emitted_factor();
    }

    fn is_using_primitive_area(&self) -> bool { self.use_primitive_area }

    fn get_type(&self) -> MaterialType { unimplemented!() }

    fn get_event_types(&self) -> BSDFEvent { unimplemented!() }

    fn is_light_source(&self) -> bool { self.emitted_tex.is_some() }

    fn set_photon_gi_enabled(&mut self, v: bool) { self.is_photon_gi_enabled = v; }

    fn is_photon_gi_enabled(&self) -> bool { self.is_photon_gi_enabled }

    fn get_glossiness(&self) -> f32 { self.glossiness }

    fn set_direct_light_sampling_type(&mut self, t: MaterialEmissionDLSType) {
        self.direct_light_sampling_type = t;
    }

    fn get_direct_light_sampling_type(&self) -> &MaterialEmissionDLSType {
        &self.direct_light_sampling_type
    }

    fn set_indirect_diffuse_visibility(&mut self, visible: bool) {
        self.is_visible_indirect_diffuse = visible
    }

    fn is_visible_indirect_diffuse(&self) -> bool { self.is_visible_indirect_diffuse }

    fn set_indirect_glossy_visibility(&mut self, visible: bool) {
        self.is_visible_indirect_glossy = visible
    }

    fn is_visible_indirect_glossy(&self) -> bool { self.is_visible_indirect_glossy }

    fn set_indirect_specular_visibility(&mut self, visible: bool) {
        self.is_visible_indirect_specular = visible
    }

    fn is_visible_indirect_specular(&self) -> bool { self.is_visible_indirect_specular }

    fn set_shadow_catcher(&mut self, enabled: bool) { self.is_shadow_catcher = enabled }

    fn is_shadow_catcher(&self) -> bool { self.is_shadow_catcher }

    fn set_shadow_catcher_only_infinite_lights(&mut self, enabled: bool) {
        self.is_shadow_cater_only_infinite_lights = enabled
    }

    fn is_shadow_catcher_only_infinite_lights(&self) -> bool {
        self.is_shadow_cater_only_infinite_lights
    }

    fn set_holdout(&mut self, enabled: bool) { self.is_holdout = enabled }

    fn is_holdout(&self) -> bool { self.is_holdout }

    fn set_bump_sample_distance(&mut self, dist: f32) { self.bump_sample_distance = dist }

    fn get_bump_sample_distance(&self) -> f32 { self.bump_sample_distance }

    fn is_delta(&self) -> bool { false }

    fn get_avg_pass_through_transparency(&self) -> f32 { self.avg_pass_through_transparency }

    fn get_pass_through_transparency(
        &self,
        hit_point: &HitPoint,
        local_fixed_dir: Vector,
        pass_through_event: f32,
        back_tracing: bool,
    ) -> Spectrum {
        todo!()
    }

    fn set_pass_through_shadow_transparency(&mut self, t: Spectrum) {
        self.pass_through_shadow_transparency = t
    }

    fn get_pass_through_shadow_transparency(&self) -> &Spectrum {
        &self.pass_through_shadow_transparency
    }

    fn get_emitted_radiance(&self, hit_point: &HitPoint, one_over_primitive_area: f32) -> Spectrum {
        todo!()
    }

    fn get_emitted_radiance_y(&self, one_over_primitive_area: f32) -> f32 { todo!() }

    fn set_emitted_importance(&mut self, imp: f32) { self.emitted_importance = imp; }

    fn get_emitted_importance(&self) -> f32 { self.emitted_importance }

    fn get_front_transparency_texture(&self) -> &Box<dyn Texture> { &self.front_transparency_tex }

    fn get_back_transparency_texture(&self) -> &Box<dyn Texture> { &self.back_transparency_tex }

    fn get_emit_texture(&self) -> &Option<Box<dyn Texture>> { &self.emitted_tex }

    fn get_bump_texture(&self) -> &Option<Box<dyn Texture>> { &self.bump_tax }

    fn set_emission_map(&mut self, map: ImageMap) { self.emission_map = map }

    fn get_emission_map(&self) -> &ImageMap { &self.emission_map }

    fn get_emission_func(&self) -> &SampleableSphericalFunction { &self.emission_func }

    fn set_interior_volume(&mut self, vol: Volume) { self.interior_volume = Box::new(vol) }

    fn get_interior_volume(&self, hit_point: &HitPoint, pass_through_event: f32) -> &Volume {
        todo!()
    }

    fn set_exterior_volume(&mut self, vol: Volume) { self.exterior_volume = Box::new(vol) }

    fn get_exterior_volume(&self, hit_point: &HitPoint, pass_through_event: f32) -> &Volume {
        todo!()
    }

    fn bump(&mut self, hit_point: &HitPoint) { todo!() }

    fn albedo(&self, hit_point: &HitPoint) -> Spectrum { todo!() }

    fn evaluate_total(&self, hit_point: &HitPoint) -> Spectrum { todo!() }

    fn evaluate(
        &self,
        hp: &HitPoint,
        local_light_dir: &Vector,
        local_eye_dir: &Vector,
        event: &BSDFEvent,
        direct_pd_fw: Option<f32>,
        reverse_pd_fw: Option<f32>,
    ) -> Spectrum {
        unimplemented!()
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
        unimplemented!()
    }

    fn pdf(
        &self,
        hp: &HitPoint,
        local_light_dir: &Vector,
        local_eye_dir: &Vector,
        direct_pd_fw: f32,
        reverse_pd_fw: f32,
    ) {
        unimplemented!()
    }

    fn update_material_references(
        &mut self,
        old_mat: &Box<dyn Material>,
        new_mat: &Box<dyn Material>,
    ) {
        todo!()
    }

    fn is_referencing(&self, mat: &Box<dyn Material>) -> bool { todo!() }

    fn add_referenced_materials(&mut self, v: &Vec<Box<dyn Material>>) { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) { todo!() }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) { todo!() }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        todo!()
    }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties { todo!() }
}
