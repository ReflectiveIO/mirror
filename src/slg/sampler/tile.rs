use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::engine::tile::TileWork;
use crate::slg::film::{Film, SampleResult};
use crate::slg::sampler::{SampleType, Sampler, SamplerSharedData, SamplerType, SobolSequence};

/// TilePathSamplerSharedData
pub struct TilePathSamplerSharedData;

impl SamplerSharedData for TilePathSamplerSharedData {
    fn reset(&mut self) { todo!() }
}

/// TilePath sampler
pub struct TilePathSampler {
    aa_samples: usize,
    sobol_sequence: SobolSequence,
    tile_work: TileWork,
    tile_film: Film,
    tile_x: usize,
    tile_y: usize,
    tile_pass: usize,

    sample0: f32,
    sample1: f32,
}

impl NamedObject for TilePathSampler {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}

impl Sampler for TilePathSampler {
    fn set_thread_index(&mut self, index: usize) { todo!() }

    fn get_type(&self) -> SamplerType { todo!() }

    fn get_tag(&self) -> String { todo!() }

    fn request_samples(&self, t: SampleType, size: usize) { todo!() }

    fn get_sample(&self, index: usize) -> f32 { todo!() }

    fn next_sample(&self) -> Vec<SampleResult> { todo!() }

    fn to_properties(&self) -> Properties { todo!() }
}
