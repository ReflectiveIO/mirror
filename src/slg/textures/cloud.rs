use crate::rays::color::Spectrum;
use crate::rays::geometry::{Point, Vector};
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureMapping3D, TextureType};

// Sphere for cumulus shape
pub struct CumulusSphere {
    position: Point,
    radius: f32,
}

pub struct CloudTexture {
    scale: Vector,
    sphere_centre: Point,
    radius: f32,

    cumulus: bool,
    num_spheres: u32,
    sphere_size: f32,
    spheres: CumulusSphere,

    base_fade_distance: f32,
    sharpness: f32,
    base_flatness: f32,
    variability: f32,
    omega: f32,
    first_noise_scale: f32,
    noise_offset: f32,
    turbulence_amount: f32,
    num_octaves: u32,
    mapping: Box<dyn TextureMapping3D>,
}

impl CloudTexture {
    pub fn new(
        mapping: Box<dyn TextureMapping3D>,
        radius: f32,
        first_noise_scale: f32,
        turbulence_amount: f32,
        sharpness: f32,
        variability: f32,
        base_flatness: f32,
        num_octaves: u32,
        omega: f32,
        noise_offset: f32,
        num_spheres: u32,
        sphere_size: f32,
    ) -> Self {
        todo!()
    }

    pub fn get_texture_mapping(&self) -> &Box<dyn TextureMapping3D> { &self.mapping }

    pub fn get_radius(&self) -> f32 { self.radius }

    pub fn get_num_spheres(&self) -> u32 { self.num_spheres }

    pub fn get_sphere_size(&self) -> f32 { self.sphere_size }

    pub fn get_sharpness(&self) -> f32 { self.sharpness }

    pub fn get_base_fade_distance(&self) -> f32 { self.base_fade_distance }

    pub fn get_base_flatness(&self) -> f32 { self.base_flatness }

    pub fn get_variability(&self) -> f32 { self.variability }

    pub fn get_omega(&self) -> f32 { self.omega }

    pub fn get_noise_scale(&self) -> f32 { self.first_noise_scale }

    pub fn get_noise_offset(&self) -> f32 { self.noise_offset }

    pub fn get_turbulence_amount(&self) -> f32 { self.turbulence_amount }

    pub fn get_num_octaves(&self) -> u32 { self.num_octaves }
}

impl Texture for CloudTexture {
    fn get_type(&self) -> TextureType { TextureType::CloudTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 0.5 }

    fn filter(&self) -> f32 { 0.5 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
