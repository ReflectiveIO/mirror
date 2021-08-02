use std::str::FromStr;

use downcast_rs::Downcast;

use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::film::SampleResult;

/// SamplerSharedData
///
/// Used to share sampler specific data across multiple threads
pub trait SamplerSharedData {
    fn reset(&mut self);
}

pub enum SamplerType {
    Random,
    Metropolis,
    Sobol,
    RtPathCpuSampler,
    TilePathSampler,
}

impl Default for SamplerType {
    fn default() -> Self { SamplerType::Random }
}

impl ToString for SamplerType {
    fn to_string(&self) -> String { todo!() }
}

impl FromStr for SamplerType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> { todo!() }
}

pub enum SampleType {
    PixelNormalizedOnly,
    ScreenNormalizedOnly,
    PixelNormalizedAndScreenNormalized,
}

pub trait Sampler: NamedObject + Downcast {
    fn set_thread_index(&mut self, index: usize);
    fn get_type(&self) -> SamplerType;
    fn get_tag(&self) -> String;
    fn request_samples(&self, t: SampleType, size: usize);

    // index 0 and 1 are always image x and image y
    fn get_sample(&self, index: usize) -> f32;
    fn next_sample(&self) -> Vec<SampleResult>;

    /// Transform the current object in Properties
    fn to_properties(&self) -> Properties;
}
