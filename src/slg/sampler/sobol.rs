use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::film::{Film, SampleResult};
use crate::slg::sampler::{SampleType, Sampler, SamplerSharedData, SamplerType};

/// SobolSamplerSharedData
pub struct SobolSamplerSharedData {
    pub film: Film,
    pub seed_base: usize,
    pub film_region_pixel_count: usize,

    bucket_index: usize,

    // Holds the current pass for each pixel when using adaptive sampling
    pass_per_pixel: Vec<usize>,
}

impl SamplerSharedData for SobolSamplerSharedData {
    fn reset(&mut self) { todo!() }
}

/// Sobol sampler
/// This sampler is based on Blender cycles Sobol implementation.
pub struct SobolSampler {
    shared_data: SobolSamplerSharedData,

    sobol_sequence: SobolSequence,
    adaptive_strength: f32,
    adaptive_user_importance_weight: f32,
    bucket_size: usize,
    tile_size: usize,
    super_sampling: usize,
    overlapping: usize,

    bucket_index: usize,
    pixel_offset: usize,
    pass_offset: usize,
    pass: usize,

    sample0: f32,
    sample1: f32,
}

impl NamedObject for SobolSampler {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}

impl Sampler for SobolSampler {
    fn set_thread_index(&mut self, index: usize) { todo!() }

    fn get_type(&self) -> SamplerType { todo!() }

    fn get_tag(&self) -> String { todo!() }

    fn request_samples(&self, t: SampleType, size: usize) { todo!() }

    fn get_sample(&self, index: usize) -> f32 { todo!() }

    fn next_sample(&self) -> Vec<SampleResult> { todo!() }

    fn to_properties(&self) -> Properties { todo!() }
}

/// SobolSequence
pub struct SobolSequence {
    pub rng_pass: usize,
    pub rng0: f32,
    pub rng1: f32,
    directions: Vec<usize>,
}

impl SobolSequence {
    pub fn new() -> Self { todo!() }

    pub fn request_samples(&self, size: usize) { todo!() }

    pub fn get_sample(&self, pass: usize, index: usize) -> f32 { todo!() }

    pub fn generate_direction_vectors(dimensions: usize) -> Vec<usize> { todo!() }

    fn sobol_dimension(&self, index: usize, dimension: usize) -> usize { todo!() }
}
