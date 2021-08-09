use super::material::Material;
use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::bsdf::{BSDFEvent, BSDFEventType, HitPoint};
use crate::slg::image_map::ImageMapCache;
use crate::slg::material::base::BaseMaterial;
use crate::slg::material::MaterialType;
use crate::slg::textures::Texture;

/// Disney BRDF
/// Based on "Physically Based Shading at Disney" presentet SIGGRAPH 2012
pub struct DisneyMaterial {
    base: BaseMaterial,

    base_color: Box<dyn Texture>,
    sub_surface: Box<dyn Texture>,
    roughness: Box<dyn Texture>,
    metallic: Box<dyn Texture>,
    specular: Box<dyn Texture>,
    specular_tint: Box<dyn Texture>,
    clear_coat: Box<dyn Texture>,
    clear_coat_gloss: Box<dyn Texture>,
    anisotropic: Box<dyn Texture>,
    sheen: Box<dyn Texture>,
    sheen_tint: Box<dyn Texture>,
    film_amount: Box<dyn Texture>,
    film_thickness: Box<dyn Texture>,
    film_ior: Box<dyn Texture>,
}

impl DisneyMaterial {
    pub fn new(
        front_transp: &Box<dyn Texture>,
        back_transp: &Box<dyn Texture>,
        emitted: &Box<dyn Texture>,
        bump: &Box<dyn Texture>,
        base_color: &Box<dyn Texture>,
        sub_surface: &Box<dyn Texture>,
        roughness: &Box<dyn Texture>,
        metallic: &Box<dyn Texture>,
        specular: &Box<dyn Texture>,
        specular_tint: &Box<dyn Texture>,
        clear_coat: &Box<dyn Texture>,
        clear_coat_gloss: &Box<dyn Texture>,
        anisotropic: &Box<dyn Texture>,
        sheen: &Box<dyn Texture>,
        sheen_tint: &Box<dyn Texture>,
        film_amount: &Box<dyn Texture>,
        film_thickness: &Box<dyn Texture>,
        film_ior: &Box<dyn Texture>,
    ) -> Self {
        todo!()
    }

    pub fn get_base_color(&self) -> &Box<dyn Texture> { &self.base_color }

    pub fn get_sub_surface(&self) -> &Box<dyn Texture> { &self.sub_surface }

    pub fn get_roughness(&self) -> &Box<dyn Texture> { &self.roughness }

    pub fn get_metallic(&self) -> &Box<dyn Texture> { &self.metallic }

    pub fn get_specular(&self) -> &Box<dyn Texture> { &self.specular }

    pub fn get_specular_tint(&self) -> &Box<dyn Texture> { &self.specular_tint }

    pub fn get_clear_coat(&self) -> &Box<dyn Texture> { &self.clear_coat }

    pub fn get_clear_coat_gloss(&self) -> &Box<dyn Texture> { &self.clear_coat_gloss }

    pub fn get_anisotropic(&self) -> &Box<dyn Texture> { &self.anisotropic }

    pub fn get_sheen(&self) -> &Box<dyn Texture> { &self.sheen }

    pub fn get_sheen_tint(&self) -> &Box<dyn Texture> { &self.sheen_tint }

    pub fn get_film_amount(&self) -> &Box<dyn Texture> { &self.film_amount }

    pub fn get_film_thickness(&self) -> &Box<dyn Texture> { &self.film_thickness }

    pub fn get_film_ior(&self) -> &Box<dyn Texture> { &self.film_ior }
}

impl Material for DisneyMaterial {
    fn base(&self) -> &BaseMaterial { &self.base }

    fn get_type(&self) -> MaterialType { MaterialType::Disney }

    fn get_event_types(&self) -> BSDFEvent { BSDFEventType::GLOSSY | BSDFEventType::REFLECT }

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

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) { todo!() }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        todo!()
    }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties { todo!() }
}
