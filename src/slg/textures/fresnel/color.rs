use crate::rays::color::Spectrum;
use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::bsdf::hitpoint::HitPoint;
use crate::slg::image_map::ImageMapCache;
use crate::slg::textures::fresnel::texture::FresnelTexture;
use crate::slg::textures::{Texture, TextureType};

pub struct FresnelColorTexture {
    kr: Box<dyn Texture>,
}

impl FresnelColorTexture {
    pub fn new(kr: Box<dyn Texture>) -> Self { Self { kr } }

    pub fn get_kr(&self) -> &Box<dyn Texture> { &self.kr }
}

impl Texture for FresnelColorTexture {
    fn get_type(&self) -> TextureType { TextureType::FresnelColorTex }

    fn get_float_value(&self, hp: &HitPoint) -> f32 { todo!() }

    fn get_spectrum_value(&self, hp: &HitPoint) -> Spectrum { todo!() }

    fn y(&self) -> f32 { todo!() }

    fn filter(&self) -> f32 { todo!() }

    fn add_referenced_textures(&mut self, v: &Vec<Box<dyn Texture>>) {
        self.kr.add_referenced_textures(v);
    }

    fn update_texture_references(
        &mut self,
        old_tex: &Box<dyn Texture>,
        new_tex: &Box<dyn Texture>,
    ) {
        if self.kr == old_tex {
            self.kr = new_tex.clone()
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}

impl FresnelTexture for FresnelColorTexture {
    fn evaluate(&self, point: &HitPoint, cosi: f32) -> Spectrum { todo!() }
}
