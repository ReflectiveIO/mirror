use crate::rays::device::{DeviceDescription, IntersectionDevice};
use crate::slg::engine::cpu_engine::{CPURenderEngine, CPUTileRenderEngine};
use crate::slg::engine::tile::TileRepository;
use crate::slg::engine::{Engine, EngineType};
use crate::slg::film::Film;
use crate::slg::{EditActionList, State};

pub struct TilePathCPURenderEngine {
    tile_repository: TileRepository,
}

impl TilePathCPURenderEngine {
    pub fn new() -> Self {
        TilePathCPURenderEngine {
            tile_repository: TileRepository::default(),
        }
    }
}

impl Engine for TilePathCPURenderEngine {
    fn started(&self) -> bool {
        false
    }

    fn start(&self, film: &Film) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }

    fn editing(&self) -> bool {
        todo!()
    }

    fn begin_scene_edit(&self) {
        todo!()
    }

    fn end_scene_edit(&self, edit_actions: &EditActionList) {
        todo!()
    }

    fn begin_film_edit(&self) {
        todo!()
    }

    fn end_film_edit(&self) {
        todo!()
    }

    fn paused(&self) -> bool {
        todo!()
    }

    fn pause(&self) {
        todo!()
    }

    fn resume(&self) {
        todo!()
    }

    fn done(&self) -> bool {
        false
    }

    fn wait_for_done(&self) {
        todo!()
    }

    fn update_film(&self) {
        todo!()
    }

    fn wait_for_frame(&self) {
        todo!()
    }

    fn get_type(&self) -> EngineType {
        todo!()
    }

    fn get_tag(&self) -> String {
        todo!()
    }

    fn set_seed(&self, seed: u64) {
        todo!()
    }

    fn generate_new_seed_base(&self) {
        todo!()
    }

    fn get_state(&self) -> &State {
        todo!()
    }

    fn set_state(&self, state: &State, film: &Film) {
        todo!()
    }

    fn is_material_compiled(&self) -> bool {
        todo!()
    }

    fn get_intersection_devices(&self) -> &Vec<Box<dyn IntersectionDevice>> {
        todo!()
    }

    fn get_available_device_descriptions(&self) -> &Vec<Box<dyn DeviceDescription>> {
        todo!()
    }

    fn get_pass(&self) -> i64 {
        todo!()
    }

    fn get_eye_pass(&self) -> i64 {
        todo!()
    }

    fn get_light_pass(&self) -> i64 {
        todo!()
    }

    fn get_total_sample_count(&self) -> f64 {
        todo!()
    }

    fn get_total_eye_sample_count(&self) -> f64 {
        todo!()
    }

    fn get_total_light_sample_count(&self) -> f64 {
        todo!()
    }

    fn get_total_samples_sec(&self) -> f64 {
        todo!()
    }

    fn get_total_eye_samples_sec(&self) -> f64 {
        todo!()
    }

    fn get_total_light_samples_sec(&self) -> f64 {
        todo!()
    }

    fn get_total_rays_sec(&self) -> f64 {
        todo!()
    }

    fn get_rendering_time(&self) -> f64 {
        todo!()
    }
}

impl CPURenderEngine for TilePathCPURenderEngine {}

impl CPUTileRenderEngine for TilePathCPURenderEngine {
    fn tile_repository(&self) -> &TileRepository {
        &self.tile_repository
    }
}
