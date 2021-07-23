use std::borrow::BorrowMut;
use std::cmp::{max, min};
use std::f32::consts::PI;
use std::sync::Arc;

use super::camera::Camera;
use crate::rays;
use crate::rays::epsilon::{Epsilon, MachineEpsilon, DEFAULT_EPSILON_STATIC};
use crate::rays::geometry::{rotate, scale, translate, Matrix4x4, Point, Ray, Transform, Vector};
use crate::rays::utils::{clamp, radians};
use crate::rays::Properties;
use crate::slg::cameras::{BaseCamera, CameraType};
use crate::slg::image_map::ImageMapCache;
use crate::slg::utils::PathVolumeInfo;

pub struct EnvironmentCamera {
    base: Arc<BaseCamera>,

    pub screen_offset_x: f32,
    pub screen_offset_y: f32,
    pub degrees: f32,
    screen_window: [f32; 4],
    auto_update_screen_window: bool,
    orig: Point,
    target: Point,
    ray_origin: Point,
    up: Vector,

    pixel_area: f32,
    dir: Vector,
    x: Vector,
    y: Vector,
    trans: CameraTransforms,
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
    pub fn new(orig: &Point, target: &Point, up: &Vector, sw: Option<[f32; 4]>) -> Self {
        let mut auto_update_screen_window: bool;
        let mut screen_window: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        if let Some(w) = sw {
            auto_update_screen_window = false;
            screen_window[0] = w[0];
            screen_window[1] = w[1];
            screen_window[2] = w[2];
            screen_window[3] = w[3];
        } else {
            auto_update_screen_window = true;
        }

        EnvironmentCamera {
            base: Arc::new(BaseCamera::new(CameraType::Environment)),

            screen_offset_x: 0.0,
            screen_offset_y: 0.0,
            degrees: 360.0,
            screen_window,
            auto_update_screen_window,
            orig: orig.clone(),
            target: target.clone(),
            ray_origin: Point::new(0.0, 0.0, 0.0),
            up: up.normalize(),
            pixel_area: 0.0,
            dir: Vector::default(),
            x: Vector::default(),
            y: Vector::default(),
            trans: CameraTransforms {
                camera_to_world: Transform::default(),
                screen_to_camera: Transform::default(),
                screen_to_world: Transform::default(),
                raster_to_screen: Transform::default(),
                raster_to_world: Transform::default(),
                raster_to_camera: Transform::default(),
            },
        }
    }

    pub fn get_pixel_area(&self) -> f32 { self.pixel_area }

    fn init_camera_transforms(&mut self, trans: &mut CameraTransforms) {
        // This is a trick i use from LuxCoreRender to set camera_to_world to identity
        // matrix.
        if self.orig == self.target {
            trans.camera_to_world = Transform::default();
        } else {
            let world_to_camera = Matrix4x4::look_at_rh(&self.orig, &self.target, &self.up);
            trans.camera_to_world = world_to_camera.inverse();
        }

        // Compute environment camera transformations
        trans.screen_to_camera = Transform::default();
        trans.screen_to_world = &trans.camera_to_world * &trans.screen_to_camera;

        // Compute environment camera screen transformation
        trans.raster_to_screen = translate(
            Vector::new(
                self.screen_window[0] + self.screen_offset_x,
                self.screen_window[3] + self.screen_offset_y,
                0.0,
            ) * scale(
                self.screen_window[1] - self.screen_window[0],
                self.screen_window[2] - self.screen_window[3],
                1.0,
            ) * scale(1.0 / self.base.film_width, 1.0 / self.base.film_height, 1.0),
        );

        trans.raster_to_camera = &trans.screen_to_camera * &trans.raster_to_screen;
        trans.raster_to_world = &trans.screen_to_world * &trans.raster_to_screen;
    }

    fn init_pixel_area(&mut self) {
        let x_pixel_width = (self.screen_window[1] - self.screen_window[0]) * 0.5;
        let y_pixel_height = (self.screen_window[3] - self.screen_window[2]) * 0.5;

        self.pixel_area = x_pixel_width * y_pixel_height;
    }

    fn init_ray(&self, ray: &mut Ray, film_x: f32, film_y: f32) {
        let theta: f32 = PI * (self.base.film_height - film_y - 1.0) / self.base.film_height;
        let phi: f32 =
            radians((360.0 - self.degrees) * 0.5 + self.degrees * film_x / self.base.film_width);

        ray.update(
            &self.ray_origin,
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

    fn get_bounding_box(&self) { self.compute_bounding_box(&self.orig) }

    fn get_dir(&self) -> &Vector { &self.dir }

    fn get_raster_to_camera(&self, idx: u32) -> &Transform { &self.trans.raster_to_camera }

    fn get_camera_to_world(&self, idx: u32) -> &Transform { &self.trans.camera_to_world }

    fn get_screen_to_world(&self, idx: u32) -> &Transform { &self.trans.screen_to_world }

    fn translate(&mut self, t: &Vector) {
        self.orig += t;
        self.target += t;
    }

    fn translate_left(&mut self, k: f32) {
        self.translate(&(Vector::from(self.x).normalize() * -k));
    }

    fn translate_right(&mut self, k: f32) {
        self.translate(&(Vector::from(self.x).normalize() * k))
    }

    fn translate_forward(&mut self, k: f32) { self.translate(&(self.dir * k)) }

    fn translate_backward(&mut self, k: f32) { self.translate(&(self.dir * -k)) }

    fn rotate(&mut self, angle: f32, axis: &Vector) {
        let dir: Vector = &self.target - &self.orig;
        let t: Transform = rotate(angle, axis);
        let dir: Vector = t * dir;

        // Check if the up vector is the same of view direction. If they are, skip this
        // operation (it would trigger a Singular matrix in MatrixInvert)
        if dir.normalize().dot(&self.up).abs() < 1.0 - DEFAULT_EPSILON_STATIC {
            self.target = &self.orig + dir;
        }
    }

    fn rotate_left(&mut self, angle: f32) { self.rotate(angle, &Vector::from(self.y)) }

    fn rotate_right(&mut self, angle: f32) { self.rotate(-angle, &Vector::from(self.y)) }

    fn rotate_up(&mut self, angle: f32) { self.rotate(angle, &Vector::from(self.x)) }

    fn rotate_down(&mut self, angle: f32) { self.rotate(-angle, &Vector::from(self.x)) }

    fn update(&mut self, film_width: u32, film_height: u32, film_sub_region: Option<[u32; 4]>) {
        self.base.update(film_width, film_height, film_sub_region);

        // Used to translate the camera
        self.dir = (&self.target - &self.orig).normalize();
        self.x = self.dir.cross(&self.up).normalize();
        self.y = self.x.cross(&self.dir).normalize();

        // Initialize screen information
        let frame: f32 = film_width as f32 / film_height as f32;

        // Check if i have to update screen_window
        if self.auto_update_screen_window {
            if frame < 1.0 {
                self.screen_window = [-frame, frame, -1.0, 1.0];
            } else {
                self.screen_window = [-1.0, 1.0, -1.0 / frame, 1.0 / frame];
            }
        }

        // Initialize camera transformations
        self.init_camera_transforms(&mut self.trans);

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
        vol_info: &PathVolumeInfo,
        u0: f32,
        u1: f32,
    ) {
        self.init_ray(ray, film_x, film_y);
        vol_info.add_volume(&self.base.volume);

        ray.start = MachineEpsilon::epsilon(&ray.origin);
        ray.end = self.base.clip_yon - self.base.clip_hither;
        ray.time = time;

        if self.base.motion_system.is_some() {
            *ray = self.base.motion_system.sample(ray.time) * &self.trans.camera_to_world * ray;
            // I need to normalize the direction vector again because the motion system
            // could include some kind of scale
            ray.direction = ray.direction.normalize();
        } else {
            *ray = &self.trans.camera_to_world * ray;
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

        let w = Vector::from(self.trans.camera_to_world.inverse() * ray.direction);
        let cos_theta = w.y;
        let theta = f32::min(1.0, cos_theta).acos();
        *y = self.base.film_height - 1 - (theta * self.base.film_height * PI);
        let sin_theta = clamp(1.0 - cos_theta * cos_theta, 1e-5f32, 1.0).sqrt();

        let cos_phi = -w.z / sin_theta;
        let mut phi = clamp(cos_phi, -1.0, 1.0).acos();

        if w.x >= 0.0 {
            phi = 2.0 * PI - phi;
        }
        *x = (phi - radians((360.0 - self.degrees) * 0.5)) * self.base.film_width
            / radians(self.degrees);

        return true;
    }

    fn sample_lens(&self, time: f32, u1: f32, u2: f32, p: &mut Point) -> bool {
        let lens_point = Point::new(0.0, 0.0, 0.0);

        if self.base.motion_system.is_some() {
            *p = self.base.motion_system.sample(time) * (&self.trans.camera_to_world * lens_point);
        } else {
            *p = &self.trans.camera_to_world * lens_point;
        }

        return true;
    }

    fn get_pdf(
        &self,
        eye_ray: &Ray,
        eye_distance: f32,
        film_x: f32,
        film_y: f32,
        mut pdf_w: Option<&mut f32>,
        mut flux_to_radiance_factor: Option<&mut f32>,
    ) {
        let theta: f32 = PI * (self.base.film_height as f32 - film_y - 1.0) / self.base.film_height;
        let camera_pdf_w = 1.0 / (2.0 * PI * PI * theta.sin());

        if pdf_w.is_some() {
            pdf_w = Some(&mut camera_pdf_w);
        }

        if flux_to_radiance_factor.is_some() {
            flux_to_radiance_factor = Some(&mut (camera_pdf_w / (eye_distance * eye_distance)));
        }
    }

    fn to_properties(&self, image_map_cache: &ImageMapCache, real_filename: bool) -> Properties {
        let mut props = self.base.to_properties();

        props.set("scene.camera.type", "environment");
        props.set("scene.camera.lookat.orig", &self.orig);
        props.set("scene.camera.lookat.target", &self.target);
        props.set("scene.camera.up", self.up);

        if !self.auto_update_screen_window {
            props.set("scene.camera.screen-window", &self.screen_window);
        }

        props.set("scene.camera.environment.degrees", self.degrees as f64);

        return props;
    }
}
