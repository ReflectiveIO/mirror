use std::collections::HashMap;

use crate::rays::accelerator::{Accelerator, AcceleratorType};
use crate::rays::Context;

pub struct Dataset {
    dataset_id: u32,
    context: Context,

    total_vertex_count: u64,
    total_triangle_count: u64,

    accelerators: HashMap<AcceleratorType, Box<dyn Accelerator>>,
    accelerator_type: AcceleratorType,

    preprocessed: bool,
    has_instances: bool,
    enable_instance_support: bool,
    has_motion_blur: bool,
    enable_motion_blur_support: bool,
}

impl Dataset {
    pub fn new(ctx: Option<&Context>) -> Self {
        Self {
            dataset_id: 0,
            context: Context::new(None),
            total_vertex_count: 0,
            total_triangle_count: 0,
            accelerators: HashMap::default(),
            accelerator_type: AcceleratorType::AUTO,
            preprocessed: false,
            has_instances: false,
            enable_instance_support: false,
            has_motion_blur: false,
            enable_motion_blur_support: false,
        }
    }

    pub fn get_accelerator_type(&self) -> &AcceleratorType {
        &self.accelerator_type
    }

    pub fn set_accelerator_type(&mut self, v: AcceleratorType) {
        self.accelerator_type = v
    }

    pub fn get_instance_support(&self) -> bool {
        self.enable_instance_support
    }

    pub fn requires_instance_support(&self) -> bool {
        self.enable_instance_support && self.has_instances
    }

    pub fn has_instances(&self) -> bool {
        self.has_instances
    }

    pub fn get_motion_blur_support(&self) -> bool {
        self.has_motion_blur
    }

    pub fn requires_motion_blur_support(&self) -> bool {
        self.enable_motion_blur_support && self.has_motion_blur
    }

    pub fn has_motion_blur(&self) -> bool {
        self.has_motion_blur
    }

    pub fn preprocess(&self) {}

    pub fn preprocessed(&self) -> bool {
        self.preprocessed
    }

    pub fn has_accelerator(&self, v: AcceleratorType) -> bool {
        self.accelerators.contains_key(&v)
    }

    pub fn get_accelerator(&self, v: AcceleratorType) -> Option<&Box<dyn Accelerator>> {
        self.accelerators.get(&v)
    }

    pub fn does_all_accelerators_support_update(&self) -> bool {
        false
    }
    pub fn update_accelerators(&mut self) {}

    pub fn get_total_vertex_count(&self) -> u64 {
        self.total_vertex_count
    }

    pub fn get_total_triangle_count(&self) -> u64 {
        self.total_triangle_count
    }

    pub fn get_dataset_id(&self) -> u32 {
        self.dataset_id
    }
}
