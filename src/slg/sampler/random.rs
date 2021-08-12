use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::film::{Film, SampleResult};
use crate::slg::sampler::{SampleType, Sampler, SamplerSharedData, SamplerType};

/// RandomSamplerSharedData
pub struct RandomSamplerSharedData {
    pub film: Film,
    bucket_index: usize,
}

impl RandomSamplerSharedData {
    pub fn new(film: &Film) -> Self { todo!() }

    pub fn get_new_bucket(&self, bucket_count: usize, new_bucket_index: usize) { todo!() }
}

impl SamplerSharedData for RandomSamplerSharedData {
    fn reset(&mut self) { todo!() }
}

/// Random Sampler
pub struct RandomSampler {
    shared_data: RandomSamplerSharedData,
    adaptive_strength: f32,
    adaptive_user_importance_weight: f32,
    bucket_size: usize,
    tile_size: usize,
    super_sampling: usize,
    overlapping: usize,

    sample0: f32,
    sample1: f32,

    bucket_index: usize,
    pixel_offset: usize,
    pass_offset: usize,
    pass: usize,
}

impl NamedObject for RandomSampler {
    fn get_name(&self) -> String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}

impl Sampler for RandomSampler {
    fn set_thread_index(&mut self, index: usize) { todo!() }

    fn get_type(&self) -> SamplerType { todo!() }

    fn get_tag(&self) -> String { todo!() }

    fn request_samples(&self, t: SampleType, size: usize) { todo!() }

    fn get_sample(&self, index: usize) -> f32 { todo!() }

    fn next_sample(&self) -> Vec<SampleResult> { todo!() }

    fn to_properties(&self) -> Properties { todo!() }
}
