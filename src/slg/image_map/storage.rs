use std::any::Any;
use std::ops::{Index, IndexMut};

use generic_array::{ArrayLength, GenericArray};
use rand::Rng;

use crate::rays::color::Spectrum;
use crate::rays::geometry::UV;

pub trait ImageMapStorage {
    fn select_channel(&self, t: ChannelSelectionType) -> Box<dyn ImageMapStorage>;
    fn get_storage_type(&self) -> StorageType;
    fn get_channel_count(&self) -> u32;
    fn get_memory_size(&self) -> usize;
    fn get_pixels_data(&self) -> &dyn Any;
    fn pixels_data(&mut self) -> &mut dyn Any;
    fn set_float(&mut self, index: usize, v: f32);
    fn set_spectrum(&mut self, index: usize, v: &Spectrum);

    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;

    // Methods accepting UV parameters return an interpolated value while
    // the other return the pixel value.
    fn get_float(&self, uv: &UV) -> f32;
    fn get_float_idx(&self, index: usize) -> f32;

    fn get_spectrum(&self, uv: &UV) -> &Spectrum;
    fn get_spectrum_idx(&self, index: usize) -> &Spectrum;

    fn get_alpha(&self, uv: &UV) -> f32;
    fn get_alpha_idx(&self, index: usize) -> f32;

    fn get_duv(&self, uv: &UV) -> UV;
    fn get_duv_idx(&self, index: usize) -> UV;

    fn reverse_gamma_correction(&self, gamma: f32);
}

pub struct ImageMapPixel<T, CHANNELS: ArrayLength<T>> {
    pub c: GenericArray<T, CHANNELS>,
}

impl<T, CHANNELS> From<T> for ImageMapPixel<T, CHANNELS>
where
    T: Default + From<T> + Clone,
    CHANNELS: ArrayLength<T>,
{
    fn from(v: T) -> Self {
        // let mut data: Vec<T> = Vec::with_capacity(CHANNELS::to_usize());
        // data.fill(v);
        // Self::from(data)
        let mut result = Self {
            c: GenericArray::default(),
        };
        result.c.fill(v);
        result
    }
}

impl<T, CHANNELS> From<Vec<T>> for ImageMapPixel<T, CHANNELS>
where
    T: Default + From<T> + Clone,
    CHANNELS: ArrayLength<T>,
{
    fn from(v: Vec<T>) -> Self {
        Self {
            c: GenericArray::clone_from_slice(v.as_slice()),
        }
    }
}

impl<T, CHANNELS: ArrayLength<T>> ImageMapPixel<T, CHANNELS>
where
    T: Clone,
    CHANNELS: ArrayLength<T>,
{
    pub fn get_channel_count(&self) -> usize { CHANNELS::to_usize() }

    pub fn get_float(&self) -> f32 { todo!() }

    pub fn get_spectrum(&self) -> Spectrum { todo!() }

    pub fn get_alpha(&self) -> f32 { todo!() }

    pub fn set(&mut self, src: Vec<T>) {
        for i in 0..CHANNELS::to_usize() {
            self.c[i] = src[i].clone();
        }
    }

    pub fn set_float(&mut self, v: f32) { todo!() }

    pub fn set_spectrum(&mut self, v: &Spectrum) { todo!() }

    pub fn reverse_gamma_correction(&self, gamma: f32) { todo!() }
}

impl<T, CHANNELS: ArrayLength<T>> Clone for ImageMapPixel<T, CHANNELS> {
    fn clone(&self) -> Self { todo!() }
}

impl<T, CHANNELS: ArrayLength<T>> Index<usize> for ImageMapPixel<T, CHANNELS> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output { todo!() }
}

impl<T, CHANNELS: ArrayLength<T>> IndexMut<usize> for ImageMapPixel<T, CHANNELS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { todo!() }
}

pub struct ImageMapStorageImpl<T, CHANNELS: ArrayLength<T>> {
    width: u32,
    height: u32,
    wrap_type: WrapType,
    pixels: ImageMapPixel<T, CHANNELS>,
}

impl<T, CHANNELS: ArrayLength<T>> Clone for ImageMapStorageImpl<T, CHANNELS> {
    fn clone(&self) -> Self { todo!() }
}

impl<T: 'static, CHANNELS: 'static + ArrayLength<T>> ImageMapStorage
    for ImageMapStorageImpl<T, CHANNELS>
{
    fn select_channel(&self, t: ChannelSelectionType) -> Box<dyn ImageMapStorage> { todo!() }

    fn get_storage_type(&self) -> StorageType { todo!() }

    fn get_channel_count(&self) -> u32 { todo!() }

    fn get_memory_size(&self) -> usize { todo!() }

    fn get_pixels_data(&self) -> &dyn Any { &self.pixels }

    fn pixels_data(&mut self) -> &mut dyn Any { &mut self.pixels }

    fn set_float(&mut self, index: usize, v: f32) { todo!() }

    fn set_spectrum(&mut self, index: usize, v: &Spectrum) { todo!() }

    fn get_width(&self) -> u32 { self.width }

    fn get_height(&self) -> u32 { self.height }

    fn get_float(&self, uv: &UV) -> f32 { todo!() }

    fn get_float_idx(&self, index: usize) -> f32 { todo!() }

    fn get_spectrum(&self, uv: &UV) -> &Spectrum { todo!() }

    fn get_spectrum_idx(&self, index: usize) -> &Spectrum { todo!() }

    fn get_alpha(&self, uv: &UV) -> f32 { todo!() }

    fn get_alpha_idx(&self, index: usize) -> f32 { todo!() }

    fn get_duv(&self, uv: &UV) -> UV { todo!() }

    fn get_duv_idx(&self, index: usize) -> UV { todo!() }

    fn reverse_gamma_correction(&self, gamma: f32) { todo!() }
}

pub enum StorageType {
    Byte,
    Half,
    Float,

    // This one isn't a real storage type and is used only as argument
    // of ImageMap constructor
    Auto,
}

pub enum ChannelSelectionType {
    Default,
    Red,
    Green,
    Blue,
    Alpha,
    Mean,
    WeightedMean,
    Rgb,
    Directx2openglNormalMap,
}

#[derive(Copy, Clone, PartialEq)]
pub enum WrapType {
    Repeat,
    Black,
    White,
    Clamp,
}
