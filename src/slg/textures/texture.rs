use crate::rays::object::NamedObject;
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

impl NamedObject for Texture {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
