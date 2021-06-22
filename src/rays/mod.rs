//! The Rays core classes are defined with this module.

pub use self::core::geometry;
pub use self::core::geometry::normalize;
pub use self::utils::clamp;
pub use self::utils::properties::Properties;

mod core;
pub mod utils;

pub fn init() {
    trace!("rays::init");
    warn!("@TODO: init OpenCL");
    warn!("@TODO: init NVIDIA CUDA");
}
