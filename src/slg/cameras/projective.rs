use std::borrow::BorrowMut;
use std::sync::Arc;

use crate::rays::geometry::{rotate, Dot, Normal, Point, Ray, Transform, Vector};
use crate::rays::Properties;
use crate::slg::cameras::camera::Camera;
use crate::slg::cameras::{BaseCamera, CameraType};
use crate::slg::image_map::ImageMapCache;
use crate::slg::utils::PathVolumeInfo;

pub struct ProjectiveCamera {
    base: Arc<BaseCamera>,
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

        let mut base = Arc::new(BaseCamera::new(t));
        base.orig = orig.clone();
        base.target = target.clone();
        base.up = up.normalize();
        base.clipping_plane_normal = Normal::new(1.0, 0.0, 0.0);
        base.focal_distance = 10.0;
        base.screen_window = screen_window;
        base.auto_update_screen_window = auto_update_screen_window;
        Self { base }
    }
}

const DEFAULT_EPSILON_STATIC: f32 = 1e-5_f32;

impl Camera for ProjectiveCamera {
    #[inline]
    fn base(&mut self) -> &mut Arc<BaseCamera> { self.base.borrow_mut() }

    fn get_type(&self) -> &CameraType { &self.base.camera_type }

    fn get_bounding_box(&self) { todo!() }

    fn get_dir(&self) -> &Vector { &self.base.dir }

    fn get_raster_to_camera(&self, idx: u32) -> Option<&Transform> {
        Some(&self.base.trans.raster_to_camera)
    }

    fn get_camera_to_world(&self, idx: u32) -> Option<&Transform> {
        Some(&self.base.trans.camera_to_world)
    }

    fn get_screen_to_world(&self, idx: u32) -> Option<&Transform> {
        Some(&self.base.trans.screen_to_world)
    }

    fn translate(&mut self, t: &Vector) {
        self.base.orig += t;
        self.base.target += t;
    }

    fn translate_left(&mut self, k: f32) { self.translate(&(-k * self.base.x.normalize())); }

    fn translate_right(&mut self, k: f32) { self.translate(&(k * self.base.x.normalize())); }

    fn translate_forward(&mut self, k: f32) { self.translate(&(k * self.base.dir)); }

    fn translate_backward(&mut self, k: f32) { self.translate(&(-k * self.base.dir)); }

    fn rotate(&mut self, angle: f32, axis: &Vector) {
        let mut dir = self.base.target - self.base.orig;
        let t = rotate(angle, axis);
        let dir: Vector = t * dir;

        // Check if the up vector is the same of view direction. If they are,
        // skip this operation (it would trigger a Singular matrix in
        // MatrixInvert)
        if dir.normalize().dot(&self.base.up).abs() < 1.0 - DEFAULT_EPSILON_STATIC {
            self.base.target = self.base.orig + dir;
        }
    }

    fn rotate_left(&mut self, angle: f32) { self.rotate(angle, &self.base.y) }

    fn rotate_right(&mut self, angle: f32) { self.rotate(-angle, &self.base.y) }

    fn rotate_up(&mut self, angle: f32) { self.rotate(angle, &self.base.x) }

    fn rotate_down(&mut self, angle: f32) { self.rotate(-angle, &self.base.x) }

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
        mut pdf_w: Option<f32>,
        mut flux_to_radiance_factor: Option<f32>,
    ) {
        todo!()
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        let mut props = self.base.to_properties();

        props.set("scene.camera.lookat.orig", &self.base.orig);
        props.set("scene.camera.lookat.target", &self.base.target);
        props.set("scene.camera.up", self.base.up);

        if !self.base.auto_update_screen_window {
            props.set("scene.camera.screen-window", self.base.screen_window);
        }

        if self.base.enable_clipping_plane {
            props.set(
                "scene.camera.clipping-plane.enable",
                self.base.enable_clipping_plane,
            );
            props.set(
                "scene.camera.clipping-plane.center",
                &self.base.clipping_plane_center,
            );
            props.set(
                "scene.camera.clipping-plane.normal",
                &self.base.clipping_plane_normal,
            );
        }

        props.set("scene.camera.lens-radius", self.base.lens_radius as f64);
        props.set(
            "scene.camera.focal-distance",
            self.base.focal_distance as f64,
        );
        props.set("scene.camera.autofocus.enable", self.base.auto_focus);

        return props;
    }
}
