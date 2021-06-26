//! The Rays core classes are defined with this module.

pub use self::core::context::Context;
pub use self::core::dataset::Dataset;
pub use self::core::geometry;
pub use self::core::geometry::normalize;
pub use self::utils::clamp;
pub use self::utils::properties::Properties;

pub mod device {
    pub use super::core::device::Device;
    pub use super::core::device::DeviceDescription;
    pub use super::core::device::DeviceType;
    pub use super::core::hardware::HardwareDevice;
    pub use super::core::intersection::IntersectionDevice;
}

pub mod accelerator {
    pub use super::core::accelerator::Accelerator;
    pub use super::core::accelerator::AcceleratorType;
}

mod core;
pub mod utils;

pub fn init() {
    trace!("rays::init");
    warn!("@TODO: init OpenCL");
    warn!("@TODO: init NVIDIA CUDA");
}
