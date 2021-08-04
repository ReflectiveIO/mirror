use crate::rays::color::Spectrum;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::textures::{Texture, TextureMapping3D, TextureType};

pub struct DensityGridTexture {
    mapping: TextureMapping3D,
    nx: i32,
    ny: i32,
    nz: i32,
    image_map: ImageMap,
}

impl DensityGridTexture {
    pub fn new(mapping: TextureMapping3D, nx: i32, ny: i32, nz: i32, image_map: ImageMap) -> Self {
        Self {
            mapping,
            nx,
            ny,
            nz,
            image_map,
        }
    }

    pub fn get_width(&self) -> u32 { self.nx as u32 }

    pub fn get_height(&self) -> u32 { self.ny as u32 }

    pub fn get_depth(&self) -> u32 { self.nz as u32 }

    pub fn get_image_map(&self) -> &ImageMap { &self.image_map }

    pub fn get_texture_mapping(&self) -> &TextureMapping3D { &self.mapping }
}

impl Texture for DensityGridTexture {
    fn get_type(&self) -> TextureType { TextureType::DensityGridTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn add_referenced_image_maps(&mut self, v: &mut Vec<ImageMap>) {
        v.push(self.image_map.clone())
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
