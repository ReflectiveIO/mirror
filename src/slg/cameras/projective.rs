use std::borrow::BorrowMut;
use std::sync::Arc;

use delegate::delegate;

use crate::rays::geometry::{rotate, Normal, Point, Ray, Transform, Vector};
use crate::rays::Properties;
use crate::slg::cameras::camera::Camera;
use crate::slg::cameras::{BaseCamera, CameraType};
use crate::slg::image_map::ImageMapCache;
use crate::slg::utils::PathVolumeInfo;

struct CameraTransforms {
    // Note: all *ToWorld don't include camera motion blur
    pub camera_to_world: Transform,
    pub screen_to_camera: Transform,
    pub screen_to_world: Transform,
    pub raster_to_screen: Transform,
    pub raster_to_world: Transform,
    pub raster_to_camera: Transform,
}

pub struct ProjectiveCamera {
    base: Arc<BaseCamera>,

    // User defined values
    pub orig: Point,
    pub target: Point,
    pub up: Vector,

    // World clipping plane
    pub clipping_plane_center: Point,
    pub clipping_plane_normal: Normal,
    pub enable_clipping_plane: bool,

    // User defined values
    pub lens_radius: f32,
    pub focal_distance: f32,
    pub auto_focus: bool,

    screen_window: [f32; 4],
    auto_update_screen_window: bool,

    // Calculated values
    dir: Vector,
    x: Vector,
    y: Vector,
    pub trans: CameraTransforms,
}

impl ProjectiveCamera {
    pub fn new(
        t: CameraType,
        sw: Option<[f32; 4]>,
        orig: &Point,
        target: &Point,
        up: &Vector,
    ) -> Self {
        let mut auto_update_screen_window: bool;
        let mut screen_window: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        if let Some(w) = sw {
            auto_update_screen_window = false;
            screen_window = w;
        } else {
            auto_update_screen_window = true;
        }

        Self {
            base: Arc::new(BaseCamera::new(t)),
            orig: orig.clone(),
            target: target.clone(),
            up: up.normalize(),
            clipping_plane_center: Point::new(0.0, 0.0, 0.0),
            clipping_plane_normal: Normal::new(1.0, 0.0, 0.0),
            enable_clipping_plane: false,
            lens_radius: 0.0,
            focal_distance: 10.0,
            auto_focus: false,
            screen_window,
            auto_update_screen_window,
            dir: Default::default(),
            x: Default::default(),
            y: Default::default(),
            trans: CameraTransforms {
                camera_to_world: Default::default(),
                screen_to_camera: Default::default(),
                screen_to_world: Default::default(),
                raster_to_screen: Default::default(),
                raster_to_world: Default::default(),
                raster_to_camera: Default::default(),
            },
        }
    }
}

const DEFAULT_EPSILON_STATIC: f32 = 1e-5_f32;

impl Camera for ProjectiveCamera {
    #[inline]
    fn base(&mut self) -> &mut Arc<BaseCamera> { self.base.borrow_mut() }

    fn get_type(&self) -> &CameraType { &self.base.camera_type }

    fn get_bounding_box(&self) { todo!() }

    fn get_dir(&self) -> &Vector { &self.dir }

    fn get_raster_to_camera(&self, idx: u32) -> &Transform { &self.trans.raster_to_camera }

    fn get_camera_to_world(&self, idx: u32) -> &Transform { &self.trans.camera_to_world }

    fn get_screen_to_world(&self, idx: u32) -> &Transform { &self.trans.screen_to_world }

    fn translate(&mut self, t: &Vector) {
        self.orig += t;
        self.target += t;
    }

    fn translate_left(&mut self, k: f32) { self.translate(&(-k * self.x.normalize())); }

    fn translate_right(&mut self, k: f32) { self.translate(&(k * self.x.normalize())); }

    fn translate_forward(&mut self, k: f32) { self.translate(&(k * self.dir)); }

    fn translate_backward(&mut self, k: f32) { self.translate(&(-k * self.dir)); }

    fn rotate(&mut self, angle: f32, axis: &Vector) {
        let mut dir = &self.target - &self.orig;
        let t = rotate(angle, axis);
        let dir: Vector = t * dir;

        // Check if the up vector is the same of view direction. If they are,
        // skip this operation (it would trigger a Singular matrix in
        // MatrixInvert)
        if dir.normalize().dot(&self.up).abs() < 1.0 - DEFAULT_EPSILON_STATIC {
            self.target = &self.orig + dir;
        }
    }

    fn rotate_left(&mut self, angle: f32) { self.rotate(angle, &self.y) }

    fn rotate_right(&mut self, angle: f32) { self.rotate(-angle, &self.y) }

    fn rotate_up(&mut self, angle: f32) { self.rotate(angle, &self.x) }

    fn rotate_down(&mut self, angle: f32) { self.rotate(-angle, &self.x) }

    fn generate_ray(
        &self,
        time: f32,
        film_x: f32,
        film_y: f32,
        ray: &mut Ray,
        vol_info: &PathVolumeInfo,
        u0: f32,
        u1: f32,
    ) {
        todo!()
    }

    fn get_sample_position(&self, eye_ray: &Ray, film_x: &mut f32, film_y: &mut f32) -> bool {
        todo!()
    }

    fn sample_lens(&self, time: f32, u1: f32, u2: f32, lens_point: &mut Point) -> bool { todo!() }

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

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        let mut props = self.base.to_properties();

        props.set("scene.camera.lookat.orig", &self.orig);
        props.set("scene.camera.lookat.target", &self.target);
        props.set("scene.camera.up", self.up);

        if !self.auto_update_screen_window {
            props.set("scene.camera.screen-window", self.screen_window);
        }

        if self.enable_clipping_plane {
            props.set(
                "scene.camera.clipping-plane.enable",
                self.enable_clipping_plane,
            );
            props.set(
                "scene.camera.clipping-plane.center",
                &self.clipping_plane_center,
            );
            props.set(
                "scene.camera.clipping-plane.normal",
                &self.clipping_plane_normal,
            );
        }

        props.set("scene.camera.lens-radius", self.lens_radius as f64);
        props.set("scene.camera.focal-distance", self.focal_distance as f64);
        props.set("scene.camera.autofocus.enable", self.auto_focus);

        return props;
    }
}
