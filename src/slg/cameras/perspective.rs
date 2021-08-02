use std::sync::Arc;

use delegate::delegate;

use crate::rays::geometry::{Dot, Point, Ray, Transform, Vector};
use crate::rays::object::NamedObject;
use crate::rays::utils::Distribution2D;
use crate::rays::Properties;
use crate::slg::cameras::camera::{BokehDistributionType, Camera};
use crate::slg::cameras::{BaseCamera, CameraType, ProjectiveCamera};
use crate::slg::image_map::{ImageMap, ImageMapCache};
use crate::slg::utils::PathVolumeInfo;

pub struct PerspectiveCamera {
    base: Arc<BaseCamera>,
    inner: ProjectiveCamera,
}

impl PerspectiveCamera {
    pub fn new(
        t: CameraType,
        orig: &Point,
        target: &Point,
        up: &Vector,
        sw: Option<[f32; 4]>,
    ) -> Self {
        let mut inner = ProjectiveCamera::new(t, sw, orig, target, up);

        let base = inner.base().clone();
        base.field_of_view = 45.0;
        base.bokeh_distribution = BokehDistributionType::Exponential;
        base.bokeh_scale_x = 1.0;
        base.bokeh_scale_y = 1.0;

        Self { base, inner }
    }
}

impl Camera for PerspectiveCamera {
    delegate! {
        to self.inner {
            fn base(&mut self) -> &mut Arc<BaseCamera>;
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
            fn update(&mut self, film_width: u32, film_height: u32, film_sub_region: Option<[u32; 4]>);
            fn update_auto(&self);
            fn generate_ray(&self,time: f32, film_x: f32, film_y: f32, ray: &mut Ray, vol_info: &mut PathVolumeInfo, u0: f32, u1: f32);
        }
    }

    fn get_sample_position(&self, ray: &Ray, x: &mut f32, y: &mut f32) -> bool {
        let mut global_dir = Vector::default();

        if let Some(motion_system) = &self.base.motion_system {
            global_dir *= motion_system.sample(ray.time);
        }

        let cosi: f32 = ray.direction.dot(&global_dir);
        if cosi <= 0.0
            || (!ray.end.is_infinite()
                && (ray.end * cosi < self.base.clip_hither || ray.end * cosi > self.base.clip_yon))
        {
            return false;
        }

        let mut p0: Point;

        if self.base.lens_radius > 0.0 {
            p0 = ray.origin + ray.direction * (self.base.focal_distance / cosi)
        } else {
            p0 = ray.origin + ray.direction
        }

        if let Some(motion_system) = &self.base.motion_system {
            p0 *= motion_system.sample_inverse(ray.time);
        }
        p0 *= self.base.trans.raster_to_world.inverse();

        *x = p0.x;
        *y = self.base.film_height - 1 - p0.y;

        // Check if we are inside the image plane
        if (x < self.base.film_sub_region[0])
            || (x >= self.base.film_sub_region[1] + 1)
            || (y < self.base.film_sub_region[2])
            || (y >= self.base.film_sub_region[3] + 1)
        {
            return false;
        } else {
            // World arbitrary clipping plane support
            if self.base.enable_clipping_plane {
                // check if the ray end point is on the not visible side of the plane
                let endpoint = ray.end;
                if self
                    .base
                    .clipping_plane_normal
                    .dot(endpoint - &self.base.clipping_plane_center)
                    <= 0.0
                {
                    return false;
                }
                // Update ray mint/maxt
                apply_arbitrary_clipping_plane(ray);
            }
        }

        return true;
    }

    fn sample_lens(&self, time: f32, u1: f32, u2: f32, p: &mut Point) -> bool {
        let lens_point = Point::new(0.0, 0.0, 0.0);
        self.local_sample_lens(time, u1, u2, &lens_point);

        if let Some(motion_system) = &self.base.motion_system {
            *p = motion_system.sample(time) * (&self.base.trans.camera_to_world * lens_point);
        } else {
            *p = &self.base.trans.camera_to_world * lens_point;
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
        mut factor: Option<f32>,
    ) {
        let mut global_dir = self.get_dir().clone();
        if let Some(motion_system) = &self.base.motion_system {
            global_dir *= motion_system.sample(eye_ray.time);
        }

        let cos_at_camera = eye_ray.direction.dot(&global_dir);
        if cos_at_camera <= 0.0 {
            if pdf_w.is_some() {
                pdf_w = Some(0.0);
            }
            if factor.is_some() {
                factor = Some(0.0);
            }
        } else {
            let camera_pdf_w: f32 =
                1.0 / (cos_at_camera * cos_at_camera * cos_at_camera * self.base.pixel_area);

            if pdf_w.is_some() {
                pdf_w = Some(camera_pdf_w);
            }

            if factor.is_some() {
                factor = Some(camera_pdf_w / (eye_distance * eye_distance));
            }
        }
    }

    fn to_properties(&self, imc: &ImageMapCache, real_filename: bool) -> Properties {
        let mut props = self.inner.to_properties(imc, real_filename);

        props.set("scene.camera.type", "PERSPECTIVE");
        props.set(
            "scene.camera.oculus-rift.barrel-post-pro",
            self.base.enable_oculus_rift_barrel,
        );
        props.set("scene.camera.field-of-view", self.base.field_of_view as f64);
        props.set("scene.camera.bokeh.blades", self.base.bokeh_blades as f64);
        props.set("scene.camera.bokeh.power", self.base.bokeh_power as f64);
        props.set(
            "scene.camera.bokeh.distribution.type",
            self.base.bokeh_distribution.to_string(),
        );

        if self.base.bokeh_distribution_image_map.is_some() {
            let mut filename = String::new();
            if real_filename {
                filename = self
                    .base
                    .bokeh_distribution_image_map
                    .unwrap()
                    .get_name()
                    .clone();
            } else {
                filename =
                    imc.get_sequence_filename(&self.base.bokeh_distribution_image_map.unwrap())
            }
            props.set("scene.camera.bokeh.distribution.image", filename);
        }

        props.set("scene.camera.bokeh.scale.x", self.base.bokeh_scale_x as f64);
        props.set("scene.camera.bokeh.scale.y", self.base.bokeh_scale_y as f64);

        return props;
    }
}

fn apply_arbitrary_clipping_plane(p0: &Ray) { todo!() }
