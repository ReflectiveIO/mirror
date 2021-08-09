use std::sync::Arc;

use delegate::delegate;

use crate::rays::geometry::{Cross, Point, Ray, Transform, Vector};
use crate::rays::Properties;
use crate::slg::cameras::camera::Camera;
use crate::slg::cameras::{BaseCamera, CameraType, EnvironmentCamera, PerspectiveCamera};
use crate::slg::image_map::ImageMapCache;
use crate::slg::utils::PathVolumeInfo;

pub enum StereoCameraType {
    StereoPerspective,
    StereoEnvironment180,
    StereoEnvironment360,
}

pub struct StereoCamera {
    base: Arc<BaseCamera>,
    inner: PerspectiveCamera,

    pub horiz_stereo_eyes_distance: f32,
    pub horiz_stereo_lens_distance: f32,

    stereo_type: StereoCameraType,
    left_eye: Option<Box<dyn Camera>>,
    right_eye: Option<Box<dyn Camera>>,
}

impl StereoCamera {
    pub fn new(st: StereoCameraType, o: &Point, t: &Point, u: &Vector) -> Self {
        let mut inner = PerspectiveCamera::new(CameraType::Stereo, o, t, u, None);
        let base = inner.base().clone();
        Self {
            base,
            inner,
            horiz_stereo_eyes_distance: 0.0626,
            horiz_stereo_lens_distance: 0.2779,
            stereo_type: st,

            left_eye: None,
            right_eye: None,
        }
    }
}

impl Camera for StereoCamera {
    delegate! {
        to self.inner {
            fn base(&mut self) -> &mut Arc<BaseCamera>;
            fn get_type(&self) -> &CameraType;

            fn get_bounding_box(&self);
            fn get_dir(&self) -> &Vector;

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
            fn update_auto(&self);
        }
    }

    fn get_raster_to_camera(&self, idx: u32) -> Option<&Transform> {
        if idx == 0 {
            self.left_eye.unwrap().get_raster_to_camera(0)
        } else if idx == 1 {
            self.right_eye.unwrap().get_raster_to_camera(0)
        } else {
            error!("Unknown index in get_raster_to_camera(): {}", idx);
            None
        }
    }

    fn get_camera_to_world(&self, idx: u32) -> Option<&Transform> {
        if idx == 0 {
            self.left_eye.unwrap().get_camera_to_world(0)
        } else if idx == 1 {
            self.right_eye.unwrap().get_camera_to_world(0)
        } else {
            error!("Unknown index in get_camera_to_world(): {}", idx);
            None
        }
    }

    fn get_screen_to_world(&self, idx: u32) -> Option<&Transform> {
        if idx == 0 {
            self.left_eye.unwrap().get_screen_to_world(0)
        } else if idx == 1 {
            self.right_eye.unwrap().get_screen_to_world(0)
        } else {
            error!("Unknown index in get_screen_to_world(): {}", idx);
            None
        }
    }

    fn update(&mut self, film_width: u32, film_height: u32, film_sub_region: Option<[u32; 4]>) {
        // used to translate the camera
        self.base.dir = Vector::from(self.base.target - self.base.orig).normalize();
        self.base.x = self.base.dir.cross(&self.base.up).normalize();
        self.base.y = self.base.x.cross(&self.base.dir).normalize();

        match self.stereo_type {
            StereoCameraType::StereoPerspective => {
                // create left eye camera
                let mut left = PerspectiveCamera::new(
                    CameraType::Stereo,
                    self.base.orig - 0.5 * self.horiz_stereo_eyes_distance * self.base.x,
                    &self.base.target,
                    &self.base.up,
                    None,
                );
                left.base().clip_hither = self.base.clip_hither;
                left.base().clip_yon = self.base.clip_yon;
                left.base().shutter_open = self.base.shutter_open;
                left.base().shutter_close = self.base.shutter_close;
                left.base().auto_volume = self.base.auto_volume;
                left.base().volume = self.base.volume.clone();

                left.base().clipping_plane_center = self.base.clipping_plane_center;
                left.base().clipping_plane_normal = self.base.clipping_plane_normal;
                left.base().enable_clipping_plane = self.base.enable_clipping_plane;

                left.base().lens_radius = self.base.lens_radius;
                left.base().focal_distance = self.base.focal_distance;
                left.base().auto_focus = self.base.auto_focus;

                left.base().screen_offset_x = -self.horiz_stereo_lens_distance * 0.5;
                left.base().field_of_view = self.base.field_of_view;
                left.base().enable_oculus_rift_barrel = self.base.enable_oculus_rift_barrel;

                left.base().update(film_width / 2, film_height, None);
                self.left_eye = Some(Box::new(left));

                // Create right eye camera
                let mut right = PerspectiveCamera::new(
                    CameraType::Stereo,
                    &(self.base.orig + 0.5 * self.horiz_stereo_eyes_distance * self.base.x),
                    &self.base.target,
                    &self.base.up,
                    None,
                );

                right.base().clip_hither = self.base.clip_hither;
                right.base().clip_yon = self.base.clip_yon;
                right.base().shutter_open = self.base.shutter_open;
                right.base().shutter_close = self.base.shutter_close;
                right.base().auto_volume = self.base.auto_volume;
                right.base().volume = self.base.volume.clone();

                right.base().clipping_plane_center = self.base.clipping_plane_center;
                right.base().clipping_plane_normal = self.base.clipping_plane_normal;
                right.base().enable_clipping_plane = self.base.enable_clipping_plane;

                right.base().lens_radius = self.base.lens_radius;
                right.base().focal_distance = self.base.focal_distance;
                right.base().auto_focus = self.base.auto_focus;

                right.base().screen_offset_x = self.base.screen_offset_x;
                right.base().field_of_view = self.base.field_of_view;
                right.base().enable_oculus_rift_barrel = self.base.enable_oculus_rift_barrel;

                right.base().update(film_width / 2, film_height, None);
                self.right_eye = Some(Box::new(right));
            },
            StereoCameraType::StereoEnvironment180 => {
                // Create left eye camera
                let mut left = EnvironmentCamera::new(
                    self.base.orig - 0.5 * self.horiz_stereo_eyes_distance * self.base.x,
                    &self.base.target,
                    &self.base.up,
                    None,
                );
                left.base().screen_offset_x = -self.horiz_stereo_lens_distance * 0.5;
                left.base().degrees = 180.0;
                left.base().update(film_width / 2, film_height, None);
                self.left_eye = Some(Box::new(left));

                // Create right eye camera
                let mut right = EnvironmentCamera::new(
                    &(self.base.orig + 0.5 * self.horiz_stereo_eyes_distance * self.base.x),
                    &self.base.target,
                    &self.base.up,
                    None,
                );
                right.base().screen_offset_x = self.horiz_stereo_lens_distance * 0.5;
                right.base().degrees = 180.0;
                right.base().update(film_width / 2, film_height, None);
                self.right_eye = Some(Box::new(right));
            },
            StereoCameraType::StereoEnvironment360 => {
                // Create left eye camera
                let mut left = EnvironmentCamera::new(
                    self.base.orig - 0.5 * self.horiz_stereo_eyes_distance * self.base.x,
                    &self.base.target,
                    &self.base.up,
                    None,
                );
                left.base().screen_offset_x = -self.horiz_stereo_lens_distance * 0.5;
                left.base().degrees = 360.0;
                left.base().update(film_width, film_height / 2, None);
                self.left_eye = Some(Box::new(left));

                // Create right eye camera
                let mut right = EnvironmentCamera::new(
                    &(self.base.orig + 0.5 * self.horiz_stereo_eyes_distance * self.base.x),
                    &self.base.target,
                    &self.base.up,
                    None,
                );
                right.base().screen_offset_x = self.horiz_stereo_lens_distance * 0.5;
                right.base().degrees = 360.0;
                right.base().update(film_width, film_height / 2, None);
                self.right_eye = Some(Box::new(right));
            },
        }
    }

    fn generate_ray(
        &self,
        time: f32,
        film_x: f32,
        film_y: f32,
        ray: &mut Ray,
        vol_info: &mut PathVolumeInfo,
        u0: f32,
        u1: f32,
    ) {
        match self.stereo_type {
            StereoCameraType::StereoPerspective | StereoCameraType::StereoEnvironment180 => {
                if film_x < (self.base.film_width / 2) as f32 {
                    self.left_eye
                        .unwrap()
                        .generate_ray(time, film_x, film_y, ray, vol_info, u0, u1);
                } else {
                    let film_x = film_x - self.base.film_width as f32 / 2.0;
                    self.right_eye
                        .unwrap()
                        .generate_ray(time, film_x, film_y, ray, vol_info, u0, u1);
                }
            },
            StereoCameraType::StereoEnvironment360 => {
                if film_y < (self.base.film_height / 2) as f32 {
                    self.left_eye
                        .unwrap()
                        .generate_ray(time, film_x, film_y, ray, vol_info, u0, u1);
                } else {
                    let film_y = film_y - self.base.film_height as f32 / 2.0;
                    self.right_eye
                        .unwrap()
                        .generate_ray(time, film_x, film_y, ray, vol_info, u0, u1);
                }
            },
        }
    }

    fn get_sample_position(&self, eye_ray: &Ray, film_x: &mut f32, film_y: &mut f32) -> bool {
        self.left_eye
            .unwrap()
            .get_sample_position(eye_ray, film_x, film_y)
    }

    fn sample_lens(&self, time: f32, u1: f32, u2: f32, lens_point: &mut Point) -> bool {
        self.left_eye.unwrap().sample_lens(time, u1, u2, lens_point)
    }

    fn get_pdf(
        &self,
        ray: &Ray,
        distance: f32,
        film_x: f32,
        film_y: f32,
        mut pdf_w: Option<f32>,
        mut factor: Option<f32>,
    ) {
        self.left_eye
            .unwrap()
            .get_pdf(ray, distance, film_x, film_y, pdf_w, factor)
    }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties {
        let mut props = self.base.to_properties();

        props.set("scene.camera.type", "stereo");

        match self.stereo_type {
            StereoCameraType::StereoPerspective => {
                props.set("scene.camera.stereo.type", "perspective")
            },
            StereoCameraType::StereoEnvironment180 => {
                props.set("scene.camera.stereo.type", "environment_180")
            },
            StereoCameraType::StereoEnvironment360 => {
                props.set("scene.camera.stereo.type", "environment_360")
            },
        };

        props.set(
            "scene.camera.eyes-distance",
            self.horiz_stereo_eyes_distance as f64,
        );
        props.set(
            "scene.camera.lens-distance",
            self.horiz_stereo_lens_distance as f64,
        );

        return props;
    }
}
