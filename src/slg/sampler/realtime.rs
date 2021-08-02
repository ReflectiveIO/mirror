use crate::rays::object::NamedObject;
use crate::rays::Properties;
use crate::slg::film::{Film, SampleResult};
use crate::slg::sampler::{SampleType, Sampler, SamplerSharedData, SamplerType};

pub struct PixelCoord {
    pub x: usize,
    pub y: usize,
}

/// RTPathCPU specific sampler data
pub struct RTPathCPUSamplerSharedData {
    pub film: Film,
    pub step: usize,
    pub film_sub_region: [usize; 4],
    pub film_sub_region_width: usize,
    pub film_sub_region_height: usize,
    pub pixel_render_sequence: Vec<PixelCoord>,
}

impl SamplerSharedData for RTPathCPUSamplerSharedData {
    fn reset(&mut self) { todo!() }
}

/// RTPathCPU specific sampler
pub struct RTPathCPUSampler {
    shared_data: RTPathCPUSamplerSharedData,

    my_step: usize,
    frame_height: usize,
    current_x: usize,
    current_y: usize,
    lines_done: usize,
    first_frame_done: bool,
}

impl NamedObject for RTPathCPUSampler {
    fn get_name(&self) -> &String { todo!() }

    fn set_name(&mut self, name: &str) { todo!() }
}

impl Sampler for RTPathCPUSampler {
    fn set_thread_index(&mut self, index: usize) { todo!() }

    fn get_type(&self) -> SamplerType { todo!() }

    fn get_tag(&self) -> String { todo!() }

    fn request_samples(&self, t: SampleType, size: usize) { todo!() }

    fn get_sample(&self, index: usize) -> f32 { todo!() }

    fn next_sample(&self) -> Vec<SampleResult> { todo!() }

    fn to_properties(&self) -> Properties { todo!() }
}
