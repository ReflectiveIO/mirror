use downcast_rs::Downcast;

use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::bsdf::{BSDFEvent, HitPoint};
use crate::slg::core::SampleableSphericalFunction;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::material::BaseMaterial;
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
#[derive(Clone, PartialEq)]
pub enum MaterialEmissionDLSType {
    Enabled,
    Disabled,
    Auto,
}

impl Default for MaterialEmissionDLSType {
    fn default() -> Self { MaterialEmissionDLSType::Auto }
}

pub trait Material: Downcast {
    fn base(&self) -> &BaseMaterial;

    fn set_light_id(&mut self, id: u32) { self.base().set_light_id(id) }
    fn get_light_id(&self) -> u32 { self.base().get_light_id() }
    fn set_id(&mut self, id: u32) { self.base().set_id(id) }
    fn get_id(&self) -> u32 { self.base().get_id() }
    fn set_emitted_gain(&mut self, v: Spectrum) { self.base().set_emitted_gain(v) }
    fn get_emitted_gain(&self) -> &Spectrum { self.base().get_emitted_gain() }
    fn set_emitted_power(&mut self, v: f32) { self.base().set_emitted_power(v) }
    fn get_emitted_power(&self) -> f32 { self.base().get_emitted_power() }
    fn set_emitted_power_normalize(&mut self, v: bool) {
        self.base().set_emitted_power_normalize(v)
    }
    fn is_emitted_power_normalize(&self) -> bool { self.base().is_emitted_power_normalize() }
    fn set_emitted_gain_normalize(&mut self, v: bool) { self.base().set_emitted_gain_normalize(v) }
    fn is_emitted_gain_normalize(&self) -> bool { self.base().is_emitted_gain_normalize() }
    fn set_emitted_efficiency(&mut self, v: f32) { self.base().set_emitted_efficiency(v) }
    fn get_emitted_efficiency(&self) -> f32 { self.base().get_emitted_efficiency() }
    fn get_emitted_factor(&self) -> &Spectrum { self.base().get_emitted_factor() }
    fn set_emitted_theta(&mut self, theta: f32) { self.base().set_emitted_theta(theta) }
    fn get_emitted_theta(&self) -> f32 { self.base().get_emitted_theta() }
    fn get_emitted_cos_theta_max(&self) -> f32 { self.base().get_emitted_cos_theta_max() }
    fn set_emitted_temperature(&mut self, v: f32) { self.base().set_emitted_temperature(v) }
    fn set_emitted_temperature_normalize(&mut self, v: bool) {
        self.base().set_emitted_temperature_normalize(v)
    }
    fn is_using_primitive_area(&self) -> bool { self.base().is_using_primitive_area() }

    fn get_type(&self) -> MaterialType;
    fn get_event_types(&self) -> BSDFEvent;

    fn is_light_source(&self) -> bool { self.base().is_light_source() }
    fn set_photon_gi_enabled(&mut self, v: bool) { self.base().set_photon_gi_enabled(v) }
    fn is_photon_gi_enabled(&self) -> bool { self.base().is_photon_gi_enabled() }
    fn get_glossiness(&self) -> f32 { self.base().get_glossiness() }

    fn set_direct_light_sampling_type(&mut self, t: MaterialEmissionDLSType) {
        self.base().set_direct_light_sampling_type(t)
    }
    fn get_direct_light_sampling_type(&self) -> &MaterialEmissionDLSType {
        self.base().get_direct_light_sampling_type()
    }
    fn set_indirect_diffuse_visibility(&mut self, visible: bool) {
        self.base().set_indirect_diffuse_visibility(visible)
    }
    fn is_visible_indirect_diffuse(&self) -> bool { self.base().is_visible_indirect_diffuse() }
    fn set_indirect_glossy_visibility(&mut self, visible: bool) {
        self.base().set_indirect_glossy_visibility(visible)
    }
    fn is_visible_indirect_glossy(&self) -> bool { self.base().is_visible_indirect_glossy() }
    fn set_indirect_specular_visibility(&mut self, visible: bool) {
        self.base().set_indirect_specular_visibility(visible)
    }
    fn is_visible_indirect_specular(&self) -> bool { self.base().is_visible_indirect_specular() }

    fn set_shadow_catcher(&mut self, enabled: bool) { self.base().set_shadow_catcher(enabled) }
    fn is_shadow_catcher(&self) -> bool { self.base().is_shadow_catcher() }
    fn set_shadow_catcher_only_infinite_lights(&mut self, enabled: bool) {
        self.base().set_shadow_catcher_only_infinite_lights(enabled)
    }
    fn is_shadow_catcher_only_infinite_lights(&self) -> bool {
        self.base().is_shadow_catcher_only_infinite_lights()
    }

    fn set_holdout(&mut self, enabled: bool) { self.base().set_holdout(enabled) }
    fn is_holdout(&self) -> bool { self.base().is_holdout() }

    fn set_bump_sample_distance(&mut self, dist: f32) { self.base().set_bump_sample_distance(dist) }
    fn get_bump_sample_distance(&self) -> f32 { self.base().get_bump_sample_distance() }

    fn is_delta(&self) -> bool { self.base().is_delta() }
    fn get_avg_pass_through_transparency(&self) -> f32 {
        self.base().get_avg_pass_through_transparency()
    }
    fn get_pass_through_transparency(
        &self,
        hit_point: &HitPoint,
        local_fixed_dir: Vector,
        pass_through_event: f32,
        back_tracing: bool,
    ) -> Spectrum {
        self.base().get_pass_through_transparency(
            hit_point,
            local_fixed_dir,
            pass_through_event,
            back_tracing,
        )
    }

    fn set_pass_through_shadow_transparency(&mut self, t: Spectrum) {
        self.base().set_pass_through_shadow_transparency(t)
    }
    fn get_pass_through_shadow_transparency(&self) -> &Spectrum {
        self.base().get_pass_through_shadow_transparency()
    }

    fn get_emitted_radiance(&self, hit_point: &HitPoint, one_over_primitive_area: f32) -> Spectrum {
        self.base()
            .get_emitted_radiance(hit_point, one_over_primitive_area)
    }
    fn get_emitted_radiance_y(&self, one_over_primitive_area: f32) -> f32 {
        self.get_emitted_radiance_y(one_over_primitive_area)
    }

    fn set_emitted_importance(&mut self, imp: f32) { self.base().set_emitted_importance(imp) }
    fn get_emitted_importance(&self) -> f32 { self.base().get_emitted_importance() }
    fn get_front_transparency_texture(&self) -> &Box<dyn Texture> {
        self.base().get_front_transparency_texture()
    }
    fn get_back_transparency_texture(&self) -> &Box<dyn Texture> {
        self.base().get_back_transparency_texture()
    }
    fn get_emit_texture(&self) -> &Option<Box<dyn Texture>> { self.base().get_emit_texture() }
    fn get_bump_texture(&self) -> &Option<Box<dyn Texture>> { self.base().get_bump_texture() }

    fn set_emission_map(&mut self, map: ImageMap) { self.base().set_emission_map(map) }
    fn get_emission_map(&self) -> &ImageMap { self.base().get_emission_map() }
    fn get_emission_func(&self) -> &SampleableSphericalFunction { self.base().get_emission_func() }

    // MixMaterial can have multiple volumes assigned and needs
    // the pass_through_event information to be able to return the
    // correct volume.
    fn set_interior_volume(&mut self, vol: Volume) { self.base().set_interior_volume(vol) }
    fn get_interior_volume(&self, hit_point: &HitPoint, pass_through_event: f32) -> &Volume {
        self.base()
            .get_interior_volume(hit_point, pass_through_event)
    }
    fn set_exterior_volume(&mut self, vol: Volume) { self.base().set_exterior_volume(vol) }
    fn get_exterior_volume(&self, hit_point: &HitPoint, pass_through_event: f32) -> &Volume {
        self.base()
            .get_exterior_volume(hit_point, pass_through_event)
    }

    fn bump(&mut self, hit_point: &HitPoint) { self.base().bump(hit_point) }

    /// albedo() returns the material albedo. It is used for Albedo AOV
    fn albedo(&self, hit_point: &HitPoint) -> Spectrum { self.base().albedo(hit_point) }

    /// Returns the total reflection given an constant illumination
    /// over the hemisphere. It is currently used only by PhotonGICache
    /// and BakeCPU render engine.
    fn evaluate_total(&self, hit_point: &HitPoint) -> Spectrum {
        self.base().evaluate_total(hit_point)
    }

    /// Used to evaluate the material (i.e. get a color plus some
    /// related data) knowing the eye and light vector. It used by the path
    /// tracer to evaluate the material color when doing direct lighting
    /// (i.e. you know where you are coming from and where is the light source).
    /// BiDir uses this method also while connecting eye and light path
    /// vertices.
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
        old_mat: &Box<dyn Material>,
        new_mat: &Box<dyn Material>,
    ) {
        self.base().update_material_references(old_mat, new_mat)
    }

    /// Return true if the material is referencing the specified material
    fn is_referencing(&self, mat: &Box<dyn Material>) -> bool { self.base().is_referencing(mat) }
    fn add_referenced_materials(&mut self, v: &Vec<Box<dyn Material>>) {
        self.base().add_referenced_materials(v)
    }
    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.base().add_referenced_textures(v)
    }
    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        self.base().add_referenced_image_maps(v)
    }
    /// Update any reference to oldTex with newTex
    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        self.base().update_texture_references(old_tex, new_tex)
    }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties {
        self.base().to_properties(imc, real_filename)
    }

    fn update_avg_pass_through_transparency(&mut self) {
        self.base().update_avg_pass_through_transparency()
    }
}
impl_downcast!(Material);

impl NamedObject for Box<dyn Material> {
    fn get_name(&self) -> &String { &String::from("material") }

    fn set_name(&mut self, name: &str) { todo!() }
}
