use std::borrow::BorrowMut;
use std::sync::Arc;

use delegate::delegate;

use super::camera::{Camera, CameraType};
use crate::rays::geometry::{Point, Ray, Transform, Vector};
use crate::rays::Properties;
use crate::slg::cameras::{BaseCamera, ProjectiveCamera};
use crate::slg::image_map::ImageMapCache;
use crate::slg::utils::PathVolumeInfo;

pub struct OrthographicCamera {
    base: Arc<BaseCamera>,
    inner: ProjectiveCamera,

    camera_pdf: f32,
}

impl OrthographicCamera {
    pub fn new(orig: &Point, target: &Point, up: &Vector, sw: Option<[f32; 4]>) -> Self {
        let mut inner = ProjectiveCamera::new(CameraType::Orthographic, sw, orig, target, up);
        let base = inner.base().clone();
        Self {
            base,
            inner,
            camera_pdf: 0.0,
        }
    }
}

impl Camera for OrthographicCamera {
    delegate! {
        to self.inner {
            fn base(&mut self) -> &mut Arc<BaseCamera>;
            fn get_type(&self) -> &CameraType;

            /// Returns the bounding box of all possible ray origins for this camera.
            fn get_bounding_box(&self);
            fn get_dir(&self) -> &Vector;

            // Used for compiling camera information for OpenCL (and more)
            fn get_raster_to_camera(&self, idx: u32) -> &Transform;
            fn get_camera_to_world(&self, idx: u32) -> &Transform;
            fn get_screen_to_world(&self, idx: u32) -> &Transform;

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
            fn update(&mut self, film_width: u32, film_height: u32, film_sub_region: Option<[u32; 4]>);
            fn update_auto(&self);
            fn generate_ray(&self,time: f32, film_x: f32, film_y: f32, ray: &mut Ray, vol_info: &PathVolumeInfo, u0: f32, u1: f32);
        }
    }

    fn clamp_ray(&self, ray: &mut Ray) { todo!() }

    fn get_sample_position(&self, ray: &Ray, x: &mut f32, y: &mut f32) -> bool { todo!() }

    fn sample_lens(&self, time: f32, u1: f32, u2: f32, p: &mut Point) -> bool { todo!() }

    fn get_pdf(
        &self,
        eye_ray: &Ray,
        eye_distance: f32,
        film_x: f32,
        film_y: f32,
        pdf_w: Option<&mut f32>,
        flux_to_radiance_factor: Option<&mut f32>,
    ) {
        todo!()
    }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties { todo!() }
}
