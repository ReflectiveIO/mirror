use std::str::FromStr;

use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::texture::TextureType;
use crate::slg::textures::Texture;

pub enum InterpolationType {
    None,
    Linear,
    Cubic,
}

impl FromStr for InterpolationType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> { todo!() }
}

impl ToString for InterpolationType {
    fn to_string(&self) -> String { todo!() }
}

pub struct BandTexture {
    interpolation_type: InterpolationType,

    amount: Box<dyn Texture>,
    offsets: Vec<f32>,
    values: Vec<Spectrum>,
}

impl BandTexture {
    pub fn new(
        interpolation_type: InterpolationType,
        amount: Box<dyn Texture>,
        offsets: Vec<f32>,
        values: Vec<Spectrum>,
    ) -> Self {
        Self {
            interpolation_type,
            amount,
            offsets,
            values,
        }
    }

    pub fn get_interpolation_type(&self) -> &InterpolationType { &self.interpolation_type }

    pub fn get_amount_texture(&self) -> &Box<dyn Texture> { &self.amount }

    pub fn get_offsets(&self) -> &Vec<f32> { &self.offsets }

    pub fn get_values(&self) -> &Vec<Spectrum> { &self.values }
}

impl Texture for BandTexture {
    fn get_type(&self) -> TextureType { TextureType::BandTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.amount.add_referenced_textures(v)
    }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        self.amount.add_referenced_image_maps(v);
    }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        if self.amount == old_tex {
            self.amount = new_tex.clone();
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
