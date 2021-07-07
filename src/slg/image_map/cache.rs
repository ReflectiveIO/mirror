use crate::rays::Properties;
use crate::slg::image_map::{ChannelSelectionType, ImageMap, WrapType};

#[derive(Default)]
pub struct ImageMapCache {
    all_image_scale: f32,
}

pub struct StorageType;

impl ImageMapCache {
    pub fn set_image_resize(&mut self, s: f32) {
        self.all_image_scale = s
    }

    pub fn define_image_map(&self, im: &ImageMap) {}

    pub fn get_image_map(
        &self,
        filename: &str,
        gamma: f32,
        selection_ype: ChannelSelectionType,
        storage_type: StorageType,
        wrap_type: WrapType,
    ) -> ImageMap {
        ImageMap::default()
    }

    pub fn delete_image_map(&mut self, im: &ImageMap) {}

    pub fn get_sequence_filename(&self, im: &ImageMap) -> String {
        String::new()
    }

    pub fn get_image_map_index(&self, im: &ImageMap) -> usize {
        0
    }

    pub fn get_image_maps(&self) -> Vec<ImageMap> {
        vec![]
    }

    pub fn get_size(&self) -> usize {
        0
    }

    pub fn is_image_map_defined(&self, name: &str) -> bool {
        false
    }
}
