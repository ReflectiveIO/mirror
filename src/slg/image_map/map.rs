use crate::rays::object::NamedObject;
use crate::slg::image_map::{ChannelSelectionType, WrapType};

#[derive(Default, Clone, PartialEq)]
pub struct ImageMap {
    name: String,
}

impl ImageMap {
    pub fn select_channel(&self, t: ChannelSelectionType) {}

    pub fn reverse_gamma_correction(&self) {}

    pub fn alloc_image_map<T>(
        gamma: f32,
        channels: u32,
        width: u32,
        height: u32,
        wrap_type: WrapType,
    ) -> ImageMap {
        ImageMap::default()
    }

    pub fn random() -> ImageMap { todo!() }
}

impl NamedObject for ImageMap {
    fn get_name(&self) -> &String { &self.name }

    fn set_name(&mut self, name: &str) { self.name = name.to_string() }
}
