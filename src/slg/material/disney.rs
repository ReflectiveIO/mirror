use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::Texture;

use super::material::MaterialTrait;

/// Disney BRDF
/// Based on "Physically Based Shading at Disney" presentet SIGGRAPH 2012
pub struct DisneyMaterial {
    base_color: Texture,
    sub_surface: Texture,
    roughness: Texture,
    metallic: Texture,
    specular: Texture,
    specular_tint: Texture,
    clear_coat: Texture,
    clear_coat_gloss: Texture,
    anisotropic: Texture,
    sheen: Texture,
    sheen_tint: Texture,
    film_amount: Texture,
    film_thickness: Texture,
    film_ior: Texture,
}

impl DisneyMaterial {
    pub fn new(
        front_transp: &Texture,
        back_transp: &Texture,
        emitted: &Texture,
        bump: &Texture,
        base_color: &Texture,
        sub_surface: &Texture,
        roughness: &Texture,
        metallic: &Texture,
        specular: &Texture,
        specular_tint: &Texture,
        clear_coat: &Texture,
        clear_coat_gloss: &Texture,
        anisotropic: &Texture,
        sheen: &Texture,
        sheen_tint: &Texture,
        film_amount: &Texture,
        film_thickness: &Texture,
        film_ior: &Texture,
    ) -> Self {
        Self {
            base_color: Default::default(),
            sub_surface: Default::default(),
            roughness: Default::default(),
            metallic: Default::default(),
            specular: Default::default(),
            specular_tint: Default::default(),
            clear_coat: Default::default(),
            clear_coat_gloss: Default::default(),
            anisotropic: Default::default(),
            sheen: Default::default(),
            sheen_tint: Default::default(),
            film_amount: Default::default(),
            film_thickness: Default::default(),
            film_ior: Default::default(),
        }
    }

    pub fn get_base_color(&self) -> &Texture {
        &self.base_color
    }
    pub fn get_sub_surface(&self) -> &Texture {
        &self.sub_surface
    }
    pub fn get_roughness(&self) -> &Texture {
        &self.roughness
    }
    pub fn get_metallic(&self) -> &Texture {
        &self.metallic
    }
    pub fn get_specular(&self) -> &Texture {
        &self.specular
    }
    pub fn get_specular_tint(&self) -> &Texture {
        &self.specular_tint
    }
    pub fn get_clear_coat(&self) -> &Texture {
        &self.clear_coat
    }
    pub fn get_clear_coat_gloss(&self) -> &Texture {
        &self.clear_coat_gloss
    }
    pub fn get_anisotropic(&self) -> &Texture {
        &self.anisotropic
    }
    pub fn get_sheen(&self) -> &Texture {
        &self.sheen
    }
    pub fn get_sheen_tint(&self) -> &Texture {
        &self.sheen_tint
    }
    pub fn get_film_amount(&self) -> &Texture {
        &self.film_amount
    }
    pub fn get_film_thickness(&self) -> &Texture {
        &self.film_thickness
    }
    pub fn get_film_ior(&self) -> &Texture {
        &self.film_ior
    }
}

impl MaterialTrait for DisneyMaterial {
    fn get_type(&self) -> MaterialType {
        todo!()
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
