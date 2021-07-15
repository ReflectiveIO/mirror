//! The Rays core classes are defined with this module.

pub use self::core::context::Context;
pub use self::core::dataset::Dataset;
pub use self::core::{color, geometry, mesh};
pub use self::utils::properties::Properties;

pub mod device {
    pub use super::core::device::{Device, DeviceDescription, DeviceType};
    pub use super::core::hardware::HardwareDevice;
    pub use super::core::intersection::IntersectionDevice;
}

pub mod object {
    pub use super::core::named_object::*;
}

pub mod accelerator {
    pub use super::core::accelerator::{Accelerator, AcceleratorType};
}

mod core;
pub mod utils;

pub fn init() {
    trace!("rays::init");
    warn!("@TODO: init OpenCL");
    warn!("@TODO: init NVIDIA CUDA");
}
