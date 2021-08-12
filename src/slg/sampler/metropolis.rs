use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::film::SampleResult;
use crate::slg::sampler::{SampleType, Sampler, SamplerSharedData, SamplerType};

/// MetropolisSamplerSharedData
pub struct MetropolisSamplerSharedData {
    pub total_luminance: f32,
    pub sample_count: usize,
    pub no_black_sample_count: usize,

    pub last_luminance: f32,
    pub last_sample_count: usize,
    pub last_no_black_sample_count: usize,

    pub inv_luminance: f32,
    pub cool_down: bool,
}

impl SamplerSharedData for MetropolisSamplerSharedData {
    fn reset(&mut self) { todo!() }
}

pub enum MetropolisSampleType {
    Accepted,
    Rejected,
}

/// Metropolis Sampler
pub struct MetropolisSampler {
    shared_data: MetropolisSamplerSharedData,

    max_rejects: usize,
    large_mutation_probability: f32,
    image_mutation_range: f32,
    add_only_cuastics: bool,

    samples: Vec<f32>,
    sample_stamps: Vec<usize>,

    weight: f32,
    consec_rejects: usize,
    stamp: usize,

    // Data saved for the current sample
    current_stamp: usize,
    current_luminance: f32,
    current_samples: Vec<f32>,
    current_sample_stamps: Vec<usize>,
    current_sample_results: Vec<SampleResult>,

    // Used, most of the times, when not having a film
    last_sample_acceptance: MetropolisSampleType,
    last_sample_weight: f32,

    large_mutation_count: usize,
    is_large_mutation: bool,
}

impl NamedObject for MetropolisSampler {
    fn get_name(&self) -> String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}

impl Sampler for MetropolisSampler {
    fn set_thread_index(&mut self, index: usize) { todo!() }

    fn get_type(&self) -> SamplerType { todo!() }

    fn get_tag(&self) -> String { todo!() }

    fn request_samples(&self, t: SampleType, size: usize) { todo!() }

    fn get_sample(&self, index: usize) -> f32 { todo!() }

    fn next_sample(&self) -> Vec<SampleResult> { todo!() }

    fn to_properties(&self) -> Properties { todo!() }
}
