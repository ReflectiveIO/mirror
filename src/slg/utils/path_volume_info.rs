use crate::slg::bsdf::{BSDFEvent, HitPoint, BSDF};
use crate::slg::volume::Volume;

const PATH_VOLUME_INFO_SIZE: usize = 8;

#[derive(Default)]
pub struct PathVolumeInfo {
    current_volume: Option<Volume>,
    volume_list: Vec<Volume>,
    scattered_start: bool,
}

impl PathVolumeInfo {
    pub fn new() -> Self {
        Self {
            current_volume: None,
            volume_list: vec![],
            scattered_start: false,
        }
    }

    pub const fn get_current_volume(&self) -> Option<&Volume> { self.current_volume.as_ref() }

    pub const fn get_volume(&self, index: usize) -> &Volume { self.volume_list.get(index).unwrap() }

    pub const fn get_list_size(&self) -> usize { self.volume_list.len() }

    pub fn add_volume(&mut self, vol: &Option<Volume>) {
        if vol.is_none() || self.volume_list.len() == PATH_VOLUME_INFO_SIZE {
            return;
        }

        // Update the current volume. ">=" because i want to catch the last added
        // volume.
        if self.current_volume.is_none()
            || vol.unwrap().get_priority() >= self.current_volume.unwrap().get_priority()
        {
            self.current_volume = Some(vol.unwrap().clone());
        }

        // Add the volume to the list
        self.volume_list.push(vol.unwrap().clone());
    }

    pub fn remove_volume(&mut self, vol: &Option<Volume>) {
        if vol.is_none() || self.volume_list.is_empty() {
            return;
        }

        self.volume_list.retain(|&x| x != vol.unwrap());
        self.current_volume = Some(self.volume_list.last().unwrap().clone())
    }

    pub const fn simulate_remove_volume(&mut self, vol: Option<&Volume>) -> &Volume { todo!() }

    pub const fn simulate_add_volume(&mut self, vol: Option<&Volume>) -> Option<&Volume> { todo!() }

    pub fn set_scattered_start(&mut self, v: bool) { self.scattered_start = v }

    pub fn is_scattered_start(&self) -> bool { self.scattered_start }

    pub fn update(&mut self, event_type: &BSDFEvent, bsdf: &BSDF) { todo!() }

    pub fn continue_to_trace(&self, bsdf: &BSDF) -> bool { todo!() }

    pub fn set_hit_point_volumes(
        &mut self,
        hp: &HitPoint,
        mat_interior_volume: &Volume,
        mat_exterior_volume: &Volume,
        default_world_volume: &Volume,
    ) {
        todo!()
    }
}
