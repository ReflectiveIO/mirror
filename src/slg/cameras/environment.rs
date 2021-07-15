use crate::rays;
use crate::rays::geometry::{normalize, Point, Transform, Vector};
use crate::rays::Properties;
use crate::slg::cameras::CameraType;
use crate::slg::image_map::ImageMapCache;

use super::camera::CameraTrait;

pub struct EnvironmentCamera {
    pub screen_offset_x: f32,
    pub screen_offset_y: f32,
    pub degrees: f32,
    screen_window: Vec<f32>,
    auto_update_screen_window: bool,
    orig: Point,
    target: Point,
    ray_origin: Point,
    up: Vector,

    pixel_area: f32,
    dir: Vector,
    x: Vector,
    y: Vector,
    cam_trans: CameraTransforms,
}

struct CameraTransforms {
    pub camera_to_world: Transform,
    pub screen_to_camera: Transform,
    pub screen_to_world: Transform,
    pub raster_to_screen: Transform,
    pub raster_to_world: Transform,
    pub raster_to_camera: Transform,
}

impl EnvironmentCamera {
    pub fn new() -> Self {
        EnvironmentCamera {
            screen_offset_x: 0.0,
            screen_offset_y: 0.0,
            degrees: 0.0,
            screen_window: vec![],
            auto_update_screen_window: false,
            orig: Default::default(),
            target: Default::default(),
            ray_origin: Default::default(),
            up: Default::default(),
            pixel_area: 0.0,
            dir: Default::default(),
            x: Default::default(),
            y: Default::default(),
            cam_trans: CameraTransforms {
                camera_to_world: Default::default(),
                screen_to_camera: Default::default(),
                screen_to_world: Default::default(),
                raster_to_screen: Default::default(),
                raster_to_world: Default::default(),
                raster_to_camera: Default::default(),
            },
        }
    }

    pub fn get_pixel_area(&self) -> f32 {
        self.pixel_area
    }
}

impl CameraTrait for EnvironmentCamera {
    fn get_type(&self) -> CameraType {
        CameraType::ENVIRONMENT
    }

    fn get_bounding_box(&self) {
        self.compute_bounding_box(&self.orig)
    }

    fn get_dir(&self) -> &Vector {
        &self.dir
    }

    fn get_raster_to_camera(&self, idx: u32) -> &Transform {
        &self.cam_trans.raster_to_camera
    }

    fn get_camera_to_world(&self, idx: u32) -> &Transform {
        &self.cam_trans.camera_to_world
    }

    fn get_screen_to_world(&self, idx: u32) -> &Transform {
        &self.cam_trans.screen_to_world
    }

    fn translate(&mut self, t: &Vector) {
        self.orig += t;
        self.target += t;
    }

    fn translate_left(&mut self, k: f32) {
        self.translate(&(normalize(&Vector::from(self.x)) * -k));
    }

    fn translate_right(&mut self, k: f32) {
        self.translate(&(normalize(&Vector::from(self.x)) * k))
    }

    fn translate_forward(&mut self, k: f32) {
        self.translate(&(self.dir * k))
    }

    fn translate_backward(&mut self, k: f32) {
        self.translate(&(self.dir * -k))
    }

    fn rotate(&mut self, angle: f32, t: &Vector) {
        todo!()
    }

    fn rotate_left(&mut self, angle: f32) {
        self.rotate(angle, &Vector::from(self.y))
    }

    fn rotate_right(&mut self, angle: f32) {
        self.rotate(-angle, &Vector::from(self.y))
    }

    fn rotate_up(&mut self, angle: f32) {
        self.rotate(angle, &Vector::from(self.x))
    }

    fn rotate_down(&mut self, angle: f32) {
        self.rotate(-angle, &Vector::from(self.x))
    }

    fn update(&self, film_width: u32, film_height: u32, film_sub_region: [u32; 4]) {
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
