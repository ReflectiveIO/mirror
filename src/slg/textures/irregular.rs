use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{Texture, TextureType};

pub struct IrregularDataTexture {
    wave_lengths: Vec<f32>,
    data: Vec<f32>,
    resolution: f32,

    rgb: Spectrum,
    emission: bool,
}

impl IrregularDataTexture {
    pub fn new(
        n: u32,
        wave_lengths: Vec<f32>,
        data: Vec<f32>,
        resolution: f32,
        emission: bool,
    ) -> Self {
        Self {
            wave_lengths,
            data,
            resolution,
            rgb: Spectrum::default(),
            emission,
        }
    }

    pub fn get_wave_lengths(&self) -> &Vec<f32> { &self.wave_lengths }

    pub fn get_data(&self) -> &Vec<f32> { &self.data }

    pub fn get_rgb(&self) -> &Spectrum { &self.rgb }

    pub fn get_emission(&self) -> bool { self.emission }
}

impl Texture for IrregularDataTexture {
    fn get_type(&self) -> TextureType { TextureType::IrregularDataTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { self.rgb.y() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { self.rgb.clone() }

    fn y(&self) -> f32 { self.rgb.y() }

    fn filter(&self) -> f32 { self.rgb.filter() }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
