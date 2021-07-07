use crate::rays::Properties;
use crate::slg::image_map::ImageMapCache;

#[derive(Default)]
pub struct Texture;

impl Texture {
    pub fn to_properties(
        &self,
        image_map_cache: &ImageMapCache,
        real_filename: bool,
    ) -> Properties {
        todo!()
    }
}
