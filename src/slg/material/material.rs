use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::bsdf::BSDFEvent;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::Texture;
use crate::slg::volume::Volume;

pub enum MaterialType {
    Matte,
    Mirror,
    Glass,
    ArchGlass,
    Mix,
    NullMat,
    MatteTranslucent,
    Glossy2,
    Metal2,
    RoughGlass,
    Velvet,
    Cloth,
    CarPaint,
    RoughMatte,
    RoughMatteTranslucent,
    GlossyTranslucent,
    GlossyCoating,
    Disney,
    TwoSided,

    // Volumes
    HomogeneousVol,
    ClearVol,
    HeterogeneousVol,
}

/// Material emission direct light sampling type
pub enum MaterialEmissionDLSType {
    Enabled,
    Disabled,
    Auto,
}

#[derive(Default)]
pub struct Material {
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

    front_transparency_tex: Texture,
    back_transparency_tex: Texture,
    pass_through_shadow_transparency: Spectrum,
    emitted_tex: Option<Texture>,
    bump_tax: Texture,
    bump_sample_distance: f32,

    emission_map: ImageMap,
    emission_func: SampleableShericalFunction,
    interior_volume: Volume,
    exterior_volume: Volume,

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

impl Material {
    pub fn new(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
    ) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_light_id(&mut self, id: u32) {
        self.light_id = id
    }

    pub fn get_light_id(&self) -> u32 {
        self.light_id
    }

    pub fn set_id(&mut self, id: u32) {
        self.mat_id = id
    }

    pub fn get_id(&self) -> u32 {
        self.mat_id
    }

    pub fn set_emitted_gain(&mut self, v: Spectrum) {
        self.emitted_gain = v;
        self.update_emitted_factor();
    }

    pub fn get_emitted_gain(&self) -> &Spectrum {
        &self.emitted_gain
    }

    pub fn set_emitted_power(&mut self, v: f32) {
        self.emitted_power = v;
        self.update_emitted_factor();
    }

    pub fn get_emitted_power(&self) -> f32 {
        self.emitted_power
    }

    pub fn set_emitted_power_normalize(&mut self, v: bool) {
        self.emitted_power_normalize = v
    }

    pub fn is_emitted_power_normalize(&self) -> bool {
        self.emitted_power_normalize
    }

    pub fn set_emitted_gain_normalize(&mut self, v: bool) {
        self.emitted_gain_normalize = v
    }

    pub fn is_emitted_gain_normalize(&self) -> bool {
        self.emitted_gain_normalize
    }

    pub fn set_emitted_efficiency(&mut self, v: f32) {
        self.emitted_efficiency = v;
        self.update_emitted_factor();
    }

    pub fn get_emitted_efficiency(&self) -> f32 {
        self.emitted_efficiency
    }

    pub fn get_emiited_factor(&self) -> &Spectrum {
        &self.emitted_factor
    }

    pub fn set_emitted_theta(&mut self, theta: f32) {
        self.emitted_theta = theta
    }

    pub fn get_emitted_theta(&self) -> f32 {
        self.emitted_theta
    }

    pub fn get_emitted_cos_theta_max(&self) -> f32 {
        self.emitted_cos_theta_max
    }

    pub fn set_emitted_temperature(&mut self, v: f32) {
        self.emitted_temperature = v;
        self.update_emitted_factor();
    }

    pub fn set_emitted_temperature_normalize(&mut self, v: bool) {
        self.emitted_normalize_temperature = v;
        self.update_emitted_factor();
    }

    pub fn is_using_primitive_area(&self) -> bool {
        self.use_primitive_area
    }

    pub fn set_direct_light_sampling_type(&mut self, t: MaterialEmissionDLSType) {
        self.direct_light_sampling_type = t;
    }

    pub fn get_direct_light_sampling_type(&self) -> &MaterialEmissionDLSType {
        &self.direct_light_sampling_type
    }

    pub fn set_indirect_diffuse_visibility(&mut self, visible: bool) {
        self.is_visible_indirect_diffuse = visible
    }

    pub fn is_visible_indirect_diffuse(&self) -> bool {
        self.is_visible_indirect_diffuse
    }

    pub fn set_indirect_glossy_visibility(&mut self, visible: bool) {
        self.is_visible_indirect_glossy = visible
    }

    pub fn is_visible_indirect_glossy(&self) -> bool {
        self.is_visible_indirect_glossy
    }

    pub fn set_indirect_specular_visibility(&mut self, visible: bool) {
        self.is_visible_indirect_specular = visible
    }

    pub fn is_visible_indirect_specular(&self) -> bool {
        self.is_visible_indirect_specular
    }

    pub fn set_shadow_catcher(&mut self, enabled: bool) {
        self.is_shadow_catcher = enabled
    }

    pub fn is_shadow_catcher(&self) -> bool {
        self.is_shadow_catcher
    }

    pub fn set_shadow_catcher_only_infinite_lights(&mut self, enabled: bool) {
        self.is_shadow_cater_only_infinite_lights = enabled
    }

    pub fn is_shadow_catcher_only_infinite_lights(&self) -> bool {
        self.is_shadow_cater_only_infinite_lights
    }

    pub fn set_holdout(&mut self, enabled: bool) {
        self.is_holdout = enabled
    }

    pub fn is_holdout(&self) -> bool {
        self.is_holdout
    }

    pub fn set_bump_sample_distance(&mut self, dist: f32) {
        self.bump_sample_distance = dist
    }

    pub fn get_bump_sample_distance(&self) -> f23 {
        self.bump_sample_distance
    }

    pub fn set_pass_through_shadow_transparency(&mut self, t: Spectrum) {
        self.pass_through_shadow_transparency = t
    }

    pub fn get_pass_through_shadow_transparency(&self) -> &Spectrum {
        &self.pass_through_shadow_transparency
    }

    pub fn set_emitted_importance(&mut self, imp: f32) {
        self.emitted_importance = imp;
    }

    pub fn get_emitted_importance(&self) -> f32 {
        self.emitted_importance
    }

    pub fn get_front_transparency_texture(&self) -> &Texture {
        &self.front_transparency_tex
    }

    pub fn get_back_transparency_texture(&self) -> &Texture {
        &self.back_transparency_tex
    }

    pub fn get_emit_texture(&self) -> Option<&Texture> {
        *self.emitted_tex
    }

    pub fn get_bump_texture(&self) -> Option<&Texture> {
        *self.bump_tax
    }

    pub fn set_emission_map(&mut self, map: ImageMap) {
        self.emission_map = map
    }

    pub fn get_emission_map(&self) -> &ImageMap {
        &self.emission_map
    }

    pub fn get_emission_func(&self) -> SampleableSphericalFunction {
        ()
    }

    // MixMaterial can have multiple volumes assigned and needs
    // the pass_through_event information to be able to return the
    // correct volume.
    pub fn set_interior_volume(&mut self, vol: Volume) {
        self.interior_volume = vol
    }

    pub fn get_interior_volume(&self) -> &Volume {
        &self.interior_volume
    }

    pub fn set_exterior_volume(&mut self, vol: Volume) {
        self.exterior_volume = vol
    }

    pub fn get_exterior_volume(&self) -> &Volume {
        &self.exterior_volume
    }

    fn update_emitted_factor(&mut self) {}
}

impl MaterialTrait for Material {
    fn get_type(&self) -> MaterialType {
        todo!()
    }

    fn get_event_types(&self) -> BSDFEvent {
        todo!()
    }

    fn is_light_source(&self) -> bool {
        self.emitted_tex.is_some()
    }

    fn set_photon_gi_enabled(&mut self, v: bool) {
        self.is_photon_gi_enabled = v;
    }

    fn is_photon_gi_enabled(&self) -> bool {
        self.is_photon_gi_enabled
    }

    fn get_glossiness(&self) -> f32 {
        self.glossiness
    }

    fn get_avg_pass_through_transparency(&self) -> f32 {
        self.avg_pass_through_transparency
    }

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

    fn get_emitted_radiance_y(&self, one_over_primitive_area: f32) -> f32 {
        todo!()
    }

    fn get_interior_volume(&self, hit_point: &HitPoint, pass_through_event: f32) -> &Volume {
        &self.interior_volume
    }

    fn get_exterior_volume(&self, hit_point: &HitPoint, pass_through_event: f32) -> &Volume {
        &self.exterior_volume
    }

    fn bump(&mut self, hit_point: &HitPoint) {
        todo!()
    }

    fn albedo(&self, hit_point: &HitPoint) -> Spectrum {
        todo!()
    }

    fn evaluate_total(&self, hit_point: &HitPoint) -> Spectrum {
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

    fn update_material_references(
        &mut self,
        old_mat: &Box<dyn MaterialTrait>,
        new_mat: &Box<dyn MaterialTrait>,
    ) {
        todo!()
    }

    fn is_referencing(&self, mat: &Box<dyn MaterialTrait>) -> bool {
        todo!()
    }

    fn add_referenced_materials(&mut self, v: &Vec<Box<dyn MaterialTrait>>) {}

    fn add_referenced_textures(&mut self, v: &Vec<Texture>) {
        todo!()
    }

    fn add_referenced_image_maps(&mut self, v: &Vec<ImageMap>) {
        todo!()
    }

    fn update_texture_references(&mut self, old_tex: &Texture, new_tex: &Texture) {
        todo!()
    }

    fn to_properties(&self, imc: ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}

pub trait MaterialTrait {
    fn get_type(&self) -> MaterialType;
    fn get_event_types(&self) -> BSDFEvent;
    fn is_light_source(&self) -> bool {
        false
    }
    fn set_photon_gi_enabled(&mut self, v: bool) {}
    fn is_photon_gi_enabled(&self) -> bool {
        false
    }
    fn get_glossiness(&self) -> f32 {
        0.0
    }
    fn is_delta(&self) -> bool {
        false
    }
    fn get_avg_pass_through_transparency(&self) -> f32 {
        0.0
    }
    fn get_pass_through_transparency(
        &self,
        hit_point: &HitPoint,
        local_fixed_dir: Vector,
        pass_through_event: f32,
        back_tracing: bool,
    ) -> Spectrum {
        Box::new(())
    }
    fn get_emitted_radiance(&self, hit_point: &HitPoint, one_over_primitive_area: f32) -> Spectrum {
        Box::new(())
    }
    fn get_emitted_radiance_y(&self, one_over_primitive_area: f32) -> f32 {
        0.0
    }

    fn get_interior_volume(&self, hit_point: &HitPoint, pass_through_event: f32) -> &Volume {
        &Volume::default()
    }
    fn get_exterior_volume(&self, hit_point: &HitPoint, pass_through_event: f32) -> &Volume {
        &Volume::default()
    }
    fn bump(&mut self, hit_point: &HitPoint) {}

    /// albedo() returns the material albedo. It is used for Albedo AOV
    fn albedo(&self, hit_point: &HitPoint) -> Spectrum {
        Spectrum::new(())
    }

    /// Returns the total reflection given an constant illumination
    /// over the hemisphere. It is currently used only by PhotonGICache
    /// and BakeCPU render engine.
    fn evaluate_total(&self, hit_point: &HitPoint) -> Spectrum {
        Spectrum::new(())
    }

    /// Used to evaluate the material (i.e. get a color plus some
    /// related data) knowing the eye and light vector. It used by the path
    /// tracer to evaluate the material color when doing direct lighting
    /// (i.e. you know where you are coming from and where is the light source).
    /// BiDir uses this method also while connecting eye and light path vertices.
    fn evaluate(
        &self,
        hp: &HitPoint,
        local_light_dir: &Vector,
        local_eye_dir: &Vector,
        event: &BSDFEvent,
        direct_pd_fw: Option<f32>,
        reverse_pd_fw: Option<f32>,
    ) -> Spectrum;

    /// Used to obtain an outgoing direction (i.e get an outgoing direction
    /// plus some related data) knowing the incoming vector. It is used to
    /// extend a path, you know where you are coming from and want to know
    /// where to go next. It used by the path tracing to extend eye path and
    /// by BiDir to extend both eye and light path.
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
    ) -> Spectrum;

    /// Used to obtain direct and reverse PDFs while knowing
    /// the eye and light vector. It is used only by BiDir
    fn pdf(
        &self,
        hp: &HitPoint,
        local_light_dir: &Vector,
        local_eye_dir: &Vector,
        direct_pd_fw: f32,
        reverse_pd_fw: f32,
    );

    /// Update any reference to old_mat with new_mat (mostly used for updating
    /// mix material) but also to update volume reference
    /// (because volumes are just a special kind of materials)
    fn update_material_references(
        &mut self,
        old_mat: &Box<dyn MaterialTrait>,
        new_mat: &Box<dyn MaterialTrait>,
    ) {
    }

    fn is_referencing(&self, mat: &Box<dyn MaterialTrait>) -> bool {
        false
    }
    fn add_referenced_materials(&mut self, v: &Vec<Box<dyn MaterialTrait>>) {}
    fn add_referenced_textures(&mut self, v: &Vec<Texture>) {}
    fn add_referenced_image_maps(&mut self, v: &Vec<ImageMap>) {}

    fn update_texture_references(&mut self, old_tex: &Texture, new_tex: &Texture) {}

    fn to_properties(&self, imc: ImageMapCache, real_filename: bool) -> Properties {
        Properties::default()
    }

    fn update_avg_pass_through_transparency(&mut self) {}
}
