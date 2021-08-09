use std::borrow::BorrowMut;
use std::f32::consts::PI;
use std::sync::Arc;

use super::camera::Camera;
use crate::rays::epsilon::{Epsilon, MachineEpsilon, DEFAULT_EPSILON_STATIC};
use crate::rays::geometry::{rotate, Cross, Dot, Point, Ray, Transform, Vector};
use crate::rays::utils::{clamp, radians};
use crate::rays::Properties;
use crate::slg::cameras::camera::CameraTransforms;
use crate::slg::cameras::{BaseCamera, CameraType};
use crate::slg::image_map::ImageMapCache;
use crate::slg::utils::PathVolumeInfo;

pub struct EnvironmentCamera {
    base: Arc<BaseCamera>,
}

impl EnvironmentCamera {
    pub fn new(orig: &Point, target: &Point, up: &Vector, sw: Option<[f32; 4]>) -> Self {
        let mut auto_update_screen_window: bool;
        let mut screen_window: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        if let Some(w) = sw {
            auto_update_screen_window = false;
            screen_window = w;
        } else {
            auto_update_screen_window = true;
        }

        let mut base = Arc::new(BaseCamera::new(CameraType::Environment));
        base.degrees = 360.0;
        base.screen_window = screen_window;
        base.auto_update_screen_window = auto_update_screen_window;
        base.up = up.normalize();

        EnvironmentCamera { base }
    }

    pub fn get_pixel_area(&self) -> f32 { self.base.pixel_area }

    fn init_camera_transforms(&mut self, trans: &mut CameraTransforms) {
        todo!()
        // // This is a trick i use from LuxCoreRender to set camera_to_world to
        // identity // matrix.
        // if self.orig == self.target {
        //     trans.camera_to_world = Transform::default();
        // } else {
        //     let world_to_camera = look_at(&self.orig, &self.target,
        // &self.up);     trans.camera_to_world =
        // world_to_camera.inverse(); }
        //
        // // Compute environment camera transformations
        // trans.screen_to_camera = Transform::default();
        // trans.screen_to_world = &trans.camera_to_world *
        // &trans.screen_to_camera;
        //
        // // Compute environment camera screen transformation
        // trans.raster_to_screen = translate(
        //     &(Vector::new(
        //         self.screen_window[0] + self.screen_offset_x,
        //         self.screen_window[3] + self.screen_offset_y,
        //         0.0,
        //     ) * scale(
        //         self.screen_window[1] - self.screen_window[0],
        //         self.screen_window[2] - self.screen_window[3],
        //         1.0,
        //     ) * scale(
        //         1.0 / self.base.film_width as f32,
        //         1.0 / self.base.film_height as f32,
        //         1.0,
        //     )),
        // );
        //
        // trans.raster_to_camera = &trans.screen_to_camera *
        // &trans.raster_to_screen; trans.raster_to_world =
        // &trans.screen_to_world * &trans.raster_to_screen;
    }

    fn init_pixel_area(&mut self) {
        let x_pixel_width = (self.base.screen_window[1] - self.base.screen_window[0]) * 0.5;
        let y_pixel_height = (self.base.screen_window[3] - self.base.screen_window[2]) * 0.5;

        self.base.pixel_area = x_pixel_width * y_pixel_height;
    }

    fn init_ray(&self, ray: &mut Ray, film_x: f32, film_y: f32) {
        let theta: f32 =
            PI * (self.base.film_height as f32 - film_y - 1.0) / self.base.film_height as f32;
        let phi: f32 = radians(
            (360.0 - self.base.degrees) * 0.5
                + self.base.degrees * film_x / self.base.film_width as f32,
        );

        ray.update(
            &self.base.ray_origin,
            &Vector::new(
                -theta.sin() * phi.sin(),
                theta.cos(),
                -theta.sin() * phi.cos(),
            ),
        );
    }
}

impl Camera for EnvironmentCamera {
    fn base(&mut self) -> &mut Arc<BaseCamera> { self.base.borrow_mut() }

    fn get_type(&self) -> &CameraType { self.base.get_type() }

    fn get_bounding_box(&self) { self.compute_bounding_box(&self.base.orig) }

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

    fn translate_left(&mut self, k: f32) {
        self.translate(&(Vector::from(self.base.x).normalize() * -k));
    }

    fn translate_right(&mut self, k: f32) {
        self.translate(&(Vector::from(self.base.x).normalize() * k))
    }

    fn translate_forward(&mut self, k: f32) { self.translate(&(self.base.dir * k)) }

    fn translate_backward(&mut self, k: f32) { self.translate(&(self.base.dir * -k)) }

    fn rotate(&mut self, angle: f32, axis: &Vector) {
        let dir: Vector = Vector::from(self.base.target - self.base.orig);
        let t: Transform = rotate(angle, axis);
        let dir: Vector = t * dir;

        // Check if the up vector is the same of view direction. If they are, skip this
        // operation (it would trigger a Singular matrix in MatrixInvert)
        if dir.normalize().dot(&self.base.up).abs() < 1.0 - DEFAULT_EPSILON_STATIC {
            self.base.target = self.base.orig + dir;
        }
    }

    fn rotate_left(&mut self, angle: f32) { self.rotate(angle, &Vector::from(self.base.y)) }

    fn rotate_right(&mut self, angle: f32) { self.rotate(-angle, &Vector::from(self.base.y)) }

    fn rotate_up(&mut self, angle: f32) { self.rotate(angle, &Vector::from(self.base.x)) }

    fn rotate_down(&mut self, angle: f32) { self.rotate(-angle, &Vector::from(self.base.x)) }

    fn update(&mut self, film_width: u32, film_height: u32, film_sub_region: Option<[u32; 4]>) {
        self.base.update(film_width, film_height, film_sub_region);

        // Used to translate the camera
        self.base.dir = Vector::from(self.base.target - self.base.orig).normalize();
        self.base.x = self.base.dir.cross(&self.base.up).normalize();
        self.base.y = self.base.x.cross(&self.base.dir).normalize();

        // Initialize screen information
        let frame: f32 = film_width as f32 / film_height as f32;

        // Check if i have to update screen_window
        if self.base.auto_update_screen_window {
            if frame < 1.0 {
                self.base.screen_window = [-frame, frame, -1.0, 1.0];
            } else {
                self.base.screen_window = [-1.0, 1.0, -1.0 / frame, 1.0 / frame];
            }
        }

        // Initialize camera transformations
        self.init_camera_transforms(&mut self.base.trans);

        // Initialize pixel information
        self.init_pixel_area();
    }

    fn update_auto(&self) { todo!() }

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
        self.init_ray(ray, film_x, film_y);
        vol_info.add_volume(&self.base.volume);

        ray.start = MachineEpsilon::epsilon(&ray.origin);
        ray.end = self.base.clip_yon - self.base.clip_hither;
        ray.time = time;

        if let Some(motion_system) = &self.base.motion_system {
            *ray = ray * motion_system.sample(ray.time) * &self.base.trans.camera_to_world;
            // I need to normalize the direction vector again because the motion system
            // could include some kind of scale
            ray.direction = ray.direction.normalize();
        } else {
            *ray = &self.base.trans.camera_to_world * ray;
        }
    }

    fn clamp_ray(&self, ray: &mut Ray) {
        ray.start = f32::max(ray.start, self.base.clip_hither);
        ray.end = f32::min(ray.end, self.base.clip_yon);
    }

    fn get_sample_position(&self, ray: &Ray, x: &mut f32, y: &mut f32) -> bool {
        if ray.end.is_infinite()
            && (ray.end < self.base.clip_hither || ray.end > self.base.clip_yon)
        {
            return false;
        }

        let w = Vector::from(self.base.trans.camera_to_world.inverse() * ray.direction);
        let cos_theta = w.y;
        let theta = f32::min(1.0, cos_theta).acos();
        *y = self.base.film_height as f32 - 1.0 - (theta * self.base.film_height as f32 * PI);
        let sin_theta = clamp(1.0 - cos_theta * cos_theta, 1e-5f32, 1.0).sqrt();

        let cos_phi = -w.z / sin_theta;
        let mut phi = clamp(cos_phi, -1.0, 1.0).acos();

        if w.x >= 0.0 {
            phi = 2.0 * PI - phi;
        }
        *x = (phi - radians((360.0 - self.base.degrees) * 0.5)) * self.base.film_width as f32
            / radians(self.base.degrees);

        return true;
    }

    fn sample_lens(&self, time: f32, u1: f32, u2: f32, p: &mut Point) -> bool {
        let lens_point: Point = Point::default();

        if let Some(motion_system) = &self.base.motion_system {
            *p = lens_point * &self.base.trans.camera_to_world * motion_system.sample(time);
        } else {
            *p = lens_point * &self.base.trans.camera_to_world;
        }

        return true;
    }

    fn get_pdf(
        &self,
        eye_ray: &Ray,
        eye_distance: f32,
        film_x: f32,
        film_y: f32,
        mut pdf_w: Option<f32>,
        mut flux_to_radiance_factor: Option<f32>,
    ) {
        let theta: f32 =
            PI * (self.base.film_height as f32 - film_y - 1.0) / self.base.film_height as f32;
        let mut camera_pdf_w = 1.0 / (2.0 * PI * PI * theta.sin());

        if pdf_w.is_some() {
            pdf_w = Some(camera_pdf_w);
        }

        if flux_to_radiance_factor.is_some() {
            flux_to_radiance_factor = Some(camera_pdf_w / (eye_distance * eye_distance));
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        let mut props = self.base.to_properties();

        props.set("scene.camera.type", "environment");
        props.set("scene.camera.lookat.orig", &self.base.orig);
        props.set("scene.camera.lookat.target", &self.base.target);
        props.set("scene.camera.up", self.base.up);

        if !self.base.auto_update_screen_window {
            props.set("scene.camera.screen-window", &self.base.screen_window);
        }

        props.set("scene.camera.environment.degrees", self.base.degrees as f64);

        return props;
    }
}
