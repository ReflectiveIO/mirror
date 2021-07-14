use std::cmp::{max, min};

use crate::rays::Properties;
use crate::slg::film::pipeline::ImagePipeline;
use crate::slg::film::FilmOutput;

#[derive(Debug)]
pub enum FilmChannel {
    RadiancePerPixelNormalized,
    RadiancePerScreenNormalized,
    Alpha,
    ImagePipeline,
    Depth,
    Position,
    GeometryNormal,
    ShadingNormal,
    MaterialId,
    DirectDiffuse,
    DirectDiffuseReflect,
    DirectDiffuseTransmit,
    DirectGlossy,
    DirectGlossyReflect,
    DirectGlossyTransmit,
    Emission,
    IndirectDiffuse,
    IndirectDiffuseReflect,
    IndirectDiffuseTransmit,
    IndirectGlossy,
    IndirectGlossyReflect,
    IndirectGlossyTransmit,
    IndirectSpecular,
    IndirectSpecularReflect,
    IndirectSpecularTransmit,
    MaterialIdMask,
    DirectShadowMask,
    IndirectShadowMask,
    Uv,
    RayCount,
    ByMaterialId,
    Irradiance,
    ObjectId,
    ObjectIdMask,
    ByObjectId,
    SampleCount,
    Convergence,
    MaterialIdColor,
    Albedo,
    AvgShadingNormal,
    Noise,
    UserImportance,
}

type FilmChannels = Vec<FilmChannel>;

#[derive(Default)]
pub struct Film {
    width: u32,
    height: u32,
    pixel_count: u32,
    sub_region: Vec<u32>,
    image_pipelines: Vec<ImagePipeline>,
}

impl Film {
    pub fn new(width: u32, height: u32, sub_region: Vec<u32>) -> Self { Self::default() }

    pub fn set_thread_count(&mut self, count: u32) {}

    pub fn init(&mut self) {}

    pub fn initialized(&self) -> bool { false }

    pub fn resize(&mut self, width: u32, height: u32) {}

    pub fn reset(&mut self, only_counters: bool) {}

    pub fn clear(&mut self) {}

    pub fn parse(&mut self, props: &Properties) {}

    /* Dynamic settings */

    pub fn set_image_pipeline_idx(&mut self, index: usize, pipeline: ImagePipeline) {
        self.image_pipelines.push(pipeline);
    }

    pub fn set_image_pipeline(&mut self, pipeline: ImagePipeline) {
        self.image_pipelines.push(pipeline)
    }

    pub fn set_image_pipelines(&mut self, pipelines: Vec<ImagePipeline>) {
        self.image_pipelines = pipelines
    }

    pub fn get_image_pipeline_count(&self) -> usize { self.image_pipelines.len() }

    pub fn get_image_pipeline(&self, index: usize) -> Option<&ImagePipeline> {
        self.image_pipelines.get(index)
    }

    pub fn copy_dynamic_settings(&self, film: &Film) {}

    pub fn copy_halt_settings(&self, film: &Film) {}

    // ---

    pub fn get_film_y(&self, index: usize) -> f64 { 0.0 }

    pub fn get_film_max_value(&self, index: usize) -> f64 { 0.0 }

    pub fn set_film_with_args(
        &mut self,
        film: &Film,
        src_offset_x: u32,
        src_offset_y: u32,
        src_width: u32,
        src_height: u32,
        dst_offset_x: u32,
        dst_offset_y: u32,
    ) {
    }

    pub fn set_film(&mut self, film: &Film) {
        self.set_film_with_args(film, 0, 0, self.width, self.height, 0, 0);
    }

    pub fn add_film_with_args(
        &mut self,
        film: &Film,
        src_offset_x: u32,
        src_offset_y: u32,
        src_width: u32,
        src_height: u32,
        dst_offset_x: u32,
        dst_offset_y: u32,
    ) {
    }

    pub fn add_film(&mut self, film: &Film) {
        self.add_film_with_args(film, 0, 0, self.width, self.height, 0, 0);
    }

    /* Channels */

    pub fn has_channel(&self, c: FilmChannel) -> bool { false }

    pub fn get_channel_count(&self, c: FilmChannel) -> usize { 0 }

    pub fn get_channels(&self) -> FilmChannels { vec![] }

    /// These methods must be called before init()
    pub fn add_channel(&mut self, c: FilmChannel, props: &Properties) {}

    pub fn remove_channel(&mut self, c: FilmChannel) {}

    pub fn set_radiance_group_count(&mut self, count: u32) {}

    pub fn get_radiance_group_count(&self) -> u32 { 0 }

    pub fn get_mask_material_id(&self, index: usize) -> u32 { 0 }

    pub fn get_by_material_id(&self, index: usize) -> u32 { 0 }

    pub fn get_mask_object_id(&self, idx: usize) -> u32 { 0 }

    pub fn get_by_object_id(&self, idx: usize) -> u32 { 0 }

    pub fn get_channel<T>(&self, c: FilmChannel, idx: usize, execute_image_pipeline: bool) {}

    pub fn has_data_channel(&self) -> bool { false }

    pub fn has_composing_channel(&self) -> bool { false }

    pub fn async_execute_image_pipeline(&self, idx: usize) {}

    pub fn wait_async_execute_image_pipeline(&self) {}

    pub fn has_done_async_execute_image_pipeline(&self) {}

    pub fn execute_image_pipeline(&self, idx: usize) {}

    /* Outputs */

    pub fn has_output(&self, t: FilmOutput) -> bool { false }

    pub fn get_output_count(&self, t: FilmOutput) -> usize { 0 }

    pub fn get_output_size(&self, t: FilmOutput) -> usize { 0 }

    pub fn output(&self) {}

    pub fn output_with_args(
        &self,
        filename: &str,
        t: FilmOutput,
        props: &Properties,
        execute_image_pipeline: bool,
    ) {
    }

    pub fn get_output<T>(
        &self,
        t: FilmOutput,
        buffer: T,
        idx: usize,
        execute_image_pipeline: bool,
    ) {
    }

    // ---

    pub fn get_width(&self) -> u32 { self.width }

    pub fn get_height(&self) -> u32 { self.height }

    pub fn get_pixel_count(&self) -> u32 { self.pixel_count }

    pub fn get_sub_region(&self) -> &Vec<u32> { &self.sub_region }

    pub fn get_total_sample_count(&self) -> f64 { 0.0 }

    pub fn get_total_eye_sample_count(&self) -> f64 { 0.0 }

    pub fn get_total_light_sample_count(&self) -> f64 { 0.0 }

    pub fn get_total_time(&self) -> f64 { 0.0 }

    pub fn get_avg_sample_sec(&self) -> f64 { 0.0 }

    pub fn get_avg_eye_sample_sec(&self) -> f64 { 0.0 }

    pub fn get_avg_light_sample_sec(&self) -> f64 { 0.0 }

    /* Tests related methods (halt conditions, noise estimation, etc.) */

    pub fn set_convergence(&mut self, conv: f32) {}

    pub fn get_convergence(&self) -> f64 { 0.0 }

    pub fn parse_film_size(
        props: &Properties,
        film_full_width: &mut u32,
        film_full_height: &mut u32,
        film_sub_region: &mut [u32],
    ) -> bool {
        let mut width = 640;
        if props.has("image.width") {
            warn!("deprecated property image.width");
            width = props.get("image.width").unwrap_or(width);
        }
        width = props.get("film.width").unwrap_or(width);

        let mut height = 480;
        if props.has("image.height") {
            warn!("deprecated property image.height");
            height = props.get("image.height").unwrap_or(height);
        }
        height = props.get("film.height").unwrap_or(height);

        let mut sub_region: [i64; 4] = [0, 0, 0, 0];
        let mut sub_region_used = false;

        if props.has("film.subregion") {
            let prop: [i64; 4] =
                props
                    .get("film.subregion")
                    .unwrap_or([0, width - 1, 0, height - 1]);

            sub_region[0] = max(0, min(width - 1, prop[0]));
            sub_region[1] = max(0, min(width - 1, max(sub_region[0] + 1, prop[1])));
            sub_region[2] = max(0, min(height - 1, prop[2]));
            sub_region[3] = max(0, min(height - 1, max(sub_region[2] + 1, prop[3])));
            sub_region_used = true
        } else {
            sub_region[0] = 0;
            sub_region[1] = width - 1;
            sub_region[2] = 0;
            sub_region[3] = height - 1;
            sub_region_used = false;
        }

        *film_full_width = width as u32;
        *film_full_height = height as u32;

        film_sub_region[0] = sub_region[0] as u32;
        film_sub_region[1] = sub_region[1] as u32;
        film_sub_region[2] = sub_region[2] as u32;
        film_sub_region[3] = sub_region[3] as u32;

        return sub_region_used;
    }
}
