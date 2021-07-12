use crate::rays::geometry::{Transform, Vector};
use crate::rays::Properties;
use crate::slg::image_map::ImageMapCache;

use super::camera::CameraTrait;
use super::camera::CameraType;

pub struct OrthographicCamera;

impl CameraTrait for OrthographicCamera {
    fn get_type(&self) -> CameraType {
        CameraType::ORTHOGRAPHIC
    }

    fn get_bounding_box(&self) {
        todo!()
    }

    fn get_dir(&self) -> &Vector {
        todo!()
    }

    fn get_raster_to_camera(&self, idx: u32) -> &Transform {
        todo!()
    }

    fn get_camera_to_world(&self, idx: u32) -> &Transform {
        todo!()
    }

    fn get_screen_to_world(&self, idx: u32) -> &Transform {
        todo!()
    }

    fn translate(&mut self, t: &Vector) {
        todo!()
    }

    fn translate_left(&mut self, t: f32) {
        todo!()
    }

    fn translate_right(&mut self, t: f32) {
        todo!()
    }

    fn translate_forward(&mut self, t: f32) {
        todo!()
    }

    fn translate_backward(&mut self, t: f32) {
        todo!()
    }

    fn rotate(&mut self, angle: f32, t: &Vector) {
        todo!()
    }

    fn rotate_left(&mut self, angle: f32) {
        todo!()
    }

    fn rotate_right(&mut self, angle: f32) {
        todo!()
    }

    fn rotate_up(&mut self, angle: f32) {
        todo!()
    }

    fn rotate_down(&mut self, angle: f32) {
        todo!()
    }

    fn update(&self, film_width: u32, film_height: u32, film_sub_region: &[u32]) {
        todo!()
    }

    fn update_auto(&self) {
        todo!()
    }

    fn generate_ray(&self, time: f32, film_x: f32, film_y: f32, u0: f32, u1: f32) {
        todo!()
    }

    fn clamp_ray(&self) {
        todo!()
    }

    fn get_sample_position(&self, film_x: f32, film_y: f32) -> bool {
        todo!()
    }

    fn sample_lens(&self, time: f32, u1: f32, u2: f32) -> bool {
        todo!()
    }

    fn get_pdf(
        &self,
        eye_distance: f32,
        film_x: f32,
        film_y: f32,
        pdf_w: f32,
        flux_to_radiance_factor: f32,
    ) {
        todo!()
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        todo!()
    }
}
