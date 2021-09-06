use crate::rays::color::Spectrum;
use crate::rays::geometry::Normal;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureMapping2D, TextureType};

pub struct ImageMapTexture {
    image_map: ImageMap,
    mapping: Box<dyn TextureMapping2D>,
    gain: f32,

    // used for randomized tiling
    randomized_tiling: bool,
    randomized_tiling_lut: ImageMap,
    randomized_tiling_inv_lut: ImageMap,
}

impl ImageMapTexture {
    pub fn new(
        tex_name: &String,
        img: &ImageMap,
        mp: &Box<dyn TextureMapping2D>,
        g: f32,
        rt: bool,
    ) -> Self {
        todo!()
    }

    pub fn get_image_map(&self) -> &ImageMap { &self.image_map }

    pub fn get_texture_mapping(&self) -> &Box<dyn TextureMapping2D> { &self.mapping }

    pub fn get_gain(&self) -> f32 { self.gain }

    pub fn has_randomized_tiling(&self) -> bool { self.randomized_tiling }

    pub fn get_randomized_tiling_lut(&self) -> &ImageMap { &self.randomized_tiling_lut }

    pub fn get_randomized_tiling_inv_lut(&self) -> &ImageMap { &self.randomized_tiling_inv_lut }
}

impl Texture for ImageMapTexture {
    fn get_type(&self) -> TextureType { TextureType::ImageMap }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn bump(&self, hp: &HitPoint, sample_distance: f32) -> Normal { todo!() }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) { todo!() }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
