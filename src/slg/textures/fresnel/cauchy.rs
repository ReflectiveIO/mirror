use crate::slg::textures::fresnel::texture::FresnelTextureTrait;

use super::texture::FresnelTexture;
use crate::rays::Properties;

pub fn alloc_fresnel_cauchy_texture(
    props: &Properties,
    name: &str,
) -> Box<dyn FresnelTextureTrait> {
}

pub fn alloc_fresnel_abbe_texture(props: &Properties, name: &str) -> Box<dyn FresnelTextureTrait> {}
