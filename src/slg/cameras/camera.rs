use crate::rays::geometry::{Point, Ray, Transform, Vector};
use crate::rays::Properties;
use crate::slg::cameras::base::BaseCamera;
use crate::slg::image_map::ImageMapCache;
use crate::slg::utils::PathVolumeInfo;

#[derive(Clone)]
pub enum CameraType {
    Perspective,
    Orthographic,
    Stereo,
    Environment,
}

pub trait Camera {
    fn base(&self) -> &BaseCamera;
    fn base_mut(&mut self) -> &mut BaseCamera;

    fn get_type(&self) -> &CameraType;

    /// Returns the bounding box of all possible ray origins for this camera.
    fn get_bounding_box(&self);
    fn get_dir(&self) -> &Vector;

    // Used for compiling camera information for OpenCL (and more)
    fn get_raster_to_camera(&self, idx: u32) -> Option<&Transform>;
    fn get_camera_to_world(&self, idx: u32) -> Option<&Transform>;
    fn get_screen_to_world(&self, idx: u32) -> Option<&Transform>;

    // Translate
    fn translate(&mut self, t: &Vector);
    fn translate_left(&mut self, t: f32);
    fn translate_right(&mut self, t: f32);
    fn translate_forward(&mut self, t: f32);
    fn translate_backward(&mut self, t: f32);

    // Rotate
    fn rotate(&mut self, angle: f32, t: &Vector);
    fn rotate_left(&mut self, angle: f32);
    fn rotate_right(&mut self, angle: f32);
    fn rotate_up(&mut self, angle: f32);
    fn rotate_down(&mut self, angle: f32);

    // Preprocess / update methods
    fn update(&mut self, film_width: u32, film_height: u32, film_sub_region: Option<[u32; 4]>) {}
    fn update_auto(&self) {}

    // Rendering methods
    fn generate_ray(
        &self,
        time: f32,
        film_x: f32,
        film_y: f32,
        ray: &mut Ray,
        vol_info: &mut PathVolumeInfo,
        u0: f32,
        u1: f32,
    );

    // Used for connecting light paths to the camera
    fn clamp_ray(&self, ray: &mut Ray) {}
    fn get_sample_position(&self, ray: &Ray, x: &mut f32, y: &mut f32) -> bool;
    fn sample_lens(&self, time: f32, u1: f32, u2: f32, p: &mut Point) -> bool;
    fn get_pdf(
        &self,
        eye_ray: &Ray,
        eye_distance: f32,
        film_x: f32,
        film_y: f32,
        pdf_w: &mut Option<f32>,
        factor: &mut Option<f32>,
    );

    fn generate_ray_time(&self, u: f32) -> f32 { 0.0 }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties;
    fn update_volume_reference(&self) {}
    fn compute_bounding_box(&self, orig: &Point) {}
}

#[derive(Clone)]
pub struct CameraTransforms {
    // Note: all *ToWorld don't include camera motion blur
    pub camera_to_world: Transform,
    pub screen_to_camera: Transform,
    pub screen_to_world: Transform,
    pub raster_to_screen: Transform,
    pub raster_to_world: Transform,
    pub raster_to_camera: Transform,
}

#[derive(Clone)]
pub enum BokehDistributionType {
    None,
    Uniform,
    Exponential,
    InverseExponential,
    Gaussian,
    InverseGaussian,
    Triangular,
    Custom,
}

impl Default for BokehDistributionType {
    fn default() -> Self { BokehDistributionType::None }
}

impl ToString for BokehDistributionType {
    fn to_string(&self) -> String {
        match self {
            BokehDistributionType::None => String::from("NONE"),
            BokehDistributionType::Uniform => String::from("UNIFORM"),
            BokehDistributionType::Exponential => String::from("EXPONENTIAL"),
            BokehDistributionType::InverseExponential => String::from("INVERSEEXPONENTIAL"),
            BokehDistributionType::Gaussian => String::from("GAUSSIAN"),
            BokehDistributionType::InverseGaussian => String::from("INVERSEGAUSSIAN"),
            BokehDistributionType::Triangular => String::from("TRIANGULAR"),
            BokehDistributionType::Custom => String::from("CUSTOM"),
        }
    }
}
