use crate::rays::geometry::{Point, Transform, Vector};

pub struct Camera {
    clip_hither: f32,
    clip_yon: f32,
    shutter_open: f32,
    shutter_close: f32,
    auto_volume: bool,
    //volume: Volume,
    //motion_system: MotionSystem,
    film_width: u32,
    film_height: u32,
    film_sub_region: Vec<u32>,
    camera_type: CameraType,
}

pub enum CameraType {
    PERSPECTIVE,
    ORTHOGRAPHIC,
    STEREO,
    ENVIRONMENT,
}

impl Camera {
    pub fn new(t: CameraType) -> Self {
        Camera {
            clip_hither: 1e-3f32,
            clip_yon: 1e30f32,
            shutter_open: 0.0,
            shutter_close: 1.0,
            auto_volume: true,
            film_width: 0,
            film_height: 0,
            film_sub_region: vec![],
            camera_type: t,
        }
    }
}

pub trait CameraTrait {
    fn get_type(&self) -> CameraType;

    /// Returns the bounding box of all possible ray origins for this camera.
    fn get_bounding_box(&self);
    fn get_dir(&self) -> &Vector;

    // Used for compiling camera information for OpenCL (and more)
    fn get_raster_to_camera(&self, idx: u32) -> &Transform;
    fn get_camera_to_world(&self, idx: u32) -> &Transform;
    fn get_screen_to_world(&self, idx: u32) -> &Transform;

    // Mostly used by GUIs
    fn translate(&mut self, t: &Vector);
    fn translate_left(&mut self, t: f32);
    fn translate_right(&mut self, t: f32);
    fn translate_forward(&mut self, t: f32);
    fn translate_backward(&mut self, t: f32);

    // Mostly used by GUIs
    fn rotate(&mut self, angle: f32, t: &Vector);
    fn rotate_left(&mut self, angle: f32);
    fn rotate_right(&mut self, angle: f32);
    fn rotate_up(&mut self, angle: f32);
    fn rotate_down(&mut self, angle: f32);

    // Preprocess / update methods
    fn update(&self, film_width: u32, film_height: u32, film_sub_region: &[u32]) {}
    fn update_auto(&self) {}

    // Rendering methods
    fn generate_ray(&self, time: f32, film_x: f32, film_y: f32, u0: f32, u1: f32);

    // Used for connecting light paths to the camera
    fn clamp_ray(&self) {}
    fn get_sample_position(&self, film_x: f32, film_y: f32) -> bool;
    fn sample_lens(&self, time: f32, u1: f32, u2: f32) -> bool;
    fn get_pdf(
        &self,
        eye_distance: f32,
        film_x: f32,
        film_y: f32,
        pdf_w: f32,
        flux_to_radiance_factor: f32,
    );

    fn generate_ray_time(&self, u: f32) -> f32 {
        0.0
    }

    fn to_properties(&self) {}
    fn update_volume_reference(&self) {}
    fn compute_bounding_box(&self, orig: &Point) {}
}
