use generic_array::typenum::U3;
use generic_array::ArrayLength;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::rays::color::Spectrum;
use crate::rays::geometry::UV;
use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::image_map::cache::StorageType;
use crate::slg::image_map::storage::ImageMapPixel;
use crate::slg::image_map::{ChannelSelectionType, ImageMapStorage, WrapType};
pub struct ImageMap {
    gamma: f32,
    pixel_storage: Box<dyn ImageMapStorage>,

    // cached image information
    image_mean: f32,
    image_mean_y: f32,
}

impl ImageMap {
    fn new_with_storage_gamma(pixels: Box<dyn ImageMapStorage>, gamma: f32) -> ImageMap { todo!() }
}

impl ImageMap {
    pub fn new(filename: &str, gamma: f32, storage_type: StorageType, wrap_type: WrapType) -> Self {
        todo!()
    }

    pub fn preprocess(&mut self) { todo!() }

    pub fn select_channel(&self, t: ChannelSelectionType) -> Box<dyn ImageMapStorage> { todo!() }

    pub fn reverse_gamma_correction(&self) { todo!() }

    pub fn get_gamma(&self) -> f32 { self.gamma }

    pub fn get_channel_count(&self) -> u32 { self.pixel_storage.get_channel_count() }

    pub fn get_width(&self) -> u32 { self.pixel_storage.get_width() }

    pub fn get_height(&self) -> u32 { self.pixel_storage.get_height() }

    pub fn get_storage(&self) -> &Box<dyn ImageMapStorage> { &self.pixel_storage }

    pub fn storage(&mut self) -> &mut Box<dyn ImageMapStorage> { &mut self.pixel_storage }

    pub fn get_float(&self, uv: &UV) -> f32 { self.pixel_storage.get_float(uv) }

    pub fn get_spectrum(&self, uv: &UV) -> &Spectrum { self.pixel_storage.get_spectrum(uv) }

    pub fn get_alpha(&self, uv: &UV) -> f32 { self.pixel_storage.get_alpha(uv) }

    pub fn get_duv(&self, uv: &UV) -> UV { self.pixel_storage.get_duv(uv) }

    // Note: Resize() uses OpenImageIO Resize and it can return negative values
    // very high floating point pixel values (it is a classic filtering problem).
    // So Resample() should be used instead of Resize() for HDR images.
    pub fn resize(&mut self, w: u32, h: u32) { todo!() }

    pub fn get_file_extension(&self) -> String { todo!() }

    pub fn write_image(&mut self, filename: &str) { todo!() }

    pub fn get_spectrum_mean(&self) -> f32 { self.image_mean }

    pub fn get_spectrum_mean_y(&self) -> f32 { self.image_mean_y }

    pub fn alloc_image_map<T>(
        gamma: f32,
        channels: u32,
        width: u32,
        height: u32,
        wrap_type: WrapType,
    ) -> ImageMap {
        let image_map_storage = alloc_image_map_storage::<T>(channels, width, height, wrap_type);
        ImageMap::new_with_storage_gamma(image_map_storage, gamma)
    }

    pub fn to_properties(&self, prefix: &str, include_blob_img: bool) -> Properties { todo!() }

    pub fn random() -> ImageMap {
        let size: usize = 512;
        let mut random_image_map =
            ImageMap::alloc_image_map::<f32>(1.0, 3, size as u32, size as u32, WrapType::Repeat);

        // Initialized the random image map
        let mut generator = StdRng::seed_from_u64(123);
        let random_map_data = random_image_map
            .storage()
            .pixels_data()
            .downcast_mut::<ImageMapPixel<f32, U3>>()
            .unwrap();
        for i in 0..(3 * size * size) {
            random_map_data[i] = generator.gen::<f32>();
        }

        random_image_map
    }
}

fn alloc_image_map_storage<T>(
    channels: u32,
    width: u32,
    height: u32,
    wrap_type: WrapType,
) -> Box<dyn ImageMapStorage> {
    todo!()
}

impl Clone for ImageMap {
    fn clone(&self) -> Self { todo!() }
}

impl Default for ImageMap {
    fn default() -> Self { todo!() }
}

impl NamedObject for ImageMap {
    fn get_name(&self) -> String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}
