use crate::rays::geometry::{MotionSystem, Normal, Point, Transform, Vector};
use crate::rays::utils::Distribution2D;
use crate::rays::Properties;
use crate::slg::cameras::camera::{BokehDistributionType, CameraTransforms};
use crate::slg::cameras::CameraType;
use crate::slg::image_map::ImageMap;
use crate::slg::volume::Volume;

#[derive(Clone)]
pub struct BaseCamera {
    pub camera_type: CameraType,
    pub clip_hither: f32,
    pub clip_yon: f32,
    pub shutter_open: f32,
    pub shutter_close: f32,
    pub auto_volume: bool,
    pub volume: Option<Volume>,
    pub motion_system: Option<MotionSystem>,
    pub film_width: u32,
    pub film_height: u32,
    pub film_sub_region: [u32; 4],

    /* Projective */
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

    pub screen_window: [f64; 4],
    pub auto_update_screen_window: bool,

    // Calculated values
    pub dir: Vector,
    pub x: Vector,
    pub y: Vector,
    pub trans: CameraTransforms,

    /* Perspective */
    pub screen_offset_x: f32,
    pub screen_offset_y: f32,
    pub field_of_view: f32,

    pub bokeh_blades: u32,
    pub bokeh_power: u32,
    pub bokeh_distribution: BokehDistributionType,
    pub bokeh_distribution_image_map: Option<ImageMap>,
    pub bokeh_distribution_map: Option<Distribution2D>,
    pub bokeh_scale_x: f32,
    pub bokeh_scale_y: f32,

    pub enable_oculus_rift_barrel: bool,
    pub pixel_area: f32,

    /* Environment */
    pub degrees: f32,
    pub ray_origin: Point,
}

impl BaseCamera {
    pub fn new(t: CameraType) -> Self {
        BaseCamera {
            camera_type: t,
            clip_hither: 1e-3f32,
            clip_yon: 1e30f32,
            shutter_open: 0.0,
            shutter_close: 1.0,
            auto_volume: true,
            volume: None,
            motion_system: None,
            film_width: 0,
            film_height: 0,
            film_sub_region: [0, 0, 0, 0],

            orig: Point::default(),
            target: Point::default(),
            up: Vector::default(),
            clipping_plane_center: Point::default(),
            clipping_plane_normal: Normal::default(),
            enable_clipping_plane: false,
            lens_radius: 0.0,
            focal_distance: 0.0,
            auto_focus: false,
            screen_window: Default::default(),
            auto_update_screen_window: false,
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

            screen_offset_x: 0.0,
            screen_offset_y: 0.0,
            field_of_view: 0.0,
            bokeh_blades: 0,
            bokeh_power: 0,
            bokeh_distribution: BokehDistributionType::default(),
            bokeh_distribution_image_map: None,
            bokeh_distribution_map: None,
            bokeh_scale_x: 0.0,
            bokeh_scale_y: 0.0,
            enable_oculus_rift_barrel: false,
            pixel_area: 0.0,

            degrees: 0.0,
            ray_origin: Point::default(),
        }
    }

    #[inline]
    pub fn get_type(&self) -> &CameraType { &self.camera_type }

    pub fn update(&mut self, width: u32, height: u32, sub_region: Option<[u32; 4]>) {
        self.film_width = width;
        self.film_height = height;

        if let Some(region) = sub_region {
            self.film_sub_region = [region[0], region[1], region[2], region[3]]
        } else {
            self.film_sub_region = [0, width - 1, 0, height - 1]
        }
    }

    pub fn to_properties(&self) -> Properties {
        let mut props = Properties::new();

        props.set("scene.camera.clip-hither", self.clip_hither as f64);
        props.set("scene.camera.clip-yon", self.clip_yon as f64);
        props.set("scene.camera.shutter-open", self.shutter_open as f64);
        props.set("scene.camera.shutter-close", self.shutter_close as f64);
        props.set("scene.camera.auto-volume.enable", self.auto_volume);

        // if self.volume.is_some() {
        //     props.set(
        //         "scene.camera.volume",
        //         self.volume.unwrap().get_name().to_string(),
        //     )
        // }

        if self.motion_system.is_some() {
            props.merge(
                &self
                    .motion_system
                    .as_ref()
                    .unwrap()
                    .to_properties("scene.camera", false),
            );
        }

        return props;
    }
}
