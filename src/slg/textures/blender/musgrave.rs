use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::{
    BlenderMusgraveType, BlenderNoiseBasis, Texture, TextureMapping3D, TextureType,
};

pub struct BlenderMusgraveTexture {
    mapping: Box<dyn TextureMapping3D>,
    musgrave_type: BlenderMusgraveType,
    noise_basis: BlenderNoiseBasis,
    dimension: f32,
    intensity: f32,
    lacunarity: f32,
    offset: f32,
    gain: f32,
    octaves: f32,
    noise_size: f32,
    bright: f32,
    contrast: f32,
}

impl BlenderMusgraveTexture {
    pub fn new(
        mapping: Box<dyn TextureMapping3D>,
        musgrave_type: BlenderMusgraveType,
        noise_basis: BlenderNoiseBasis,
        dimension: f32,
        intensity: f32,
        lacunarity: f32,
        offset: f32,
        gain: f32,
        octaves: f32,
        noise_size: f32,
        bright: f32,
        contrast: f32,
    ) -> Self {
        Self {
            mapping,
            musgrave_type,
            noise_basis,
            dimension,
            intensity,
            lacunarity,
            offset,
            gain,
            octaves,
            noise_size,
            bright,
            contrast,
        }
    }

    pub fn get_texture_mapping(&self) -> &Box<dyn TextureMapping3D> { &self.mapping }

    pub fn get_musgrave_type(&self) -> &BlenderMusgraveType { &self.musgrave_type }

    pub fn get_noise_basis(&self) -> &BlenderNoiseBasis { &self.noise_basis }

    pub fn get_dimension(&self) -> f32 { self.dimension }

    pub fn get_intensity(&self) -> f32 { self.intensity }

    pub fn get_lacunarity(&self) -> f32 { self.lacunarity }

    pub fn get_offset(&self) -> f32 { self.offset }

    pub fn get_gain(&self) -> f32 { self.gain }

    pub fn get_octaves(&self) -> f32 { self.octaves }

    pub fn get_noise_size(&self) -> f32 { self.noise_size }

    pub fn get_bright(&self) -> f32 { self.bright }

    pub fn get_contrast(&self) -> f32 { self.contrast }
}

impl Texture for BlenderMusgraveTexture {
    fn get_type(&self) -> TextureType { TextureType::BlenderMusgrave }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { 0.5 }

    fn filter(&self) -> f32 { 0.5 }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
