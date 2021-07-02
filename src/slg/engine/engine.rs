use downcast_rs::Downcast;

use crate::rays::device::{DeviceDescription, IntersectionDevice};
use crate::slg::film::Film;
use crate::slg::{EditActionList, State};

#[derive(Eq, PartialEq)]
pub enum EngineType {
    PathOCLRenderEngine,
    LightCPURenderEngine,
    PathCPURenderEngine,
    BiDirCPURenderEngine,
    BiDirVMCPURenderEngine,
    FileSaverRenderEngine,
    RTPathOCLRenderEngine,
    TilePathCPURenderEngine,
    TilePathOCLRenderEngine,
    RTPathCPURenderEngine,
    BakeCPURenderEngine,
}

/// Base class for render engines
pub trait Engine: Downcast {
    fn started(&self) -> bool;
    fn start(&self, film: &Film);
    fn stop(&self);

    fn editing(&self) -> bool;
    fn begin_scene_edit(&self);
    fn end_scene_edit(&self, actions: &EditActionList);

    fn begin_film_edit(&self);
    fn end_film_edit(&self);

    fn paused(&self) -> bool;
    fn pause(&self);
    fn resume(&self);

    fn done(&self) -> bool;
    fn wait_for_done(&self);

    fn update_film(&self);
    fn wait_for_frame(&self);

    fn get_type(&self) -> EngineType;
    fn get_tag(&self) -> String;

    fn set_seed(&self, seed: u64);
    fn generate_new_seed_base(&self);

    fn get_state(&self) -> &State;
    fn set_state(&self, state: &State, film: &Film);

    fn is_material_compiled(&self) -> bool;

    fn get_intersection_devices(&self) -> &Vec<Box<dyn IntersectionDevice>>;
    fn get_available_device_descriptions(&self) -> &Vec<Box<dyn DeviceDescription>>;

    /* Statistics related methods */

    fn get_pass(&self) -> i64;
    fn get_eye_pass(&self) -> i64;
    fn get_light_pass(&self) -> i64;

    fn get_total_sample_count(&self) -> f64;
    fn get_total_eye_sample_count(&self) -> f64;
    fn get_total_light_sample_count(&self) -> f64;

    fn get_total_samples_sec(&self) -> f64;
    fn get_total_eye_samples_sec(&self) -> f64;
    fn get_total_light_samples_sec(&self) -> f64;

    fn get_total_rays_sec(&self) -> f64;
    fn get_rendering_time(&self) -> f64;
}
impl_downcast!(Engine);
