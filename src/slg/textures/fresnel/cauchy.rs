use super::texture::FresnelTexture;
use crate::rays::Properties;

pub fn alloc_fresnel_cauchy_texture(props: &Properties, name: &str) -> Box<dyn FresnelTexture> {
    todo!()
}

pub fn alloc_fresnel_abbe_texture(props: &Properties, name: &str) -> Box<dyn FresnelTexture> {
    todo!()
}
