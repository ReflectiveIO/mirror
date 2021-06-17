//! The Rays core classes are defined with this module.

pub use self::core::geometry;
pub use self::core::geometry::normalize;
pub use self::utils::clamp;
pub use self::utils::properties;

mod core;
pub mod utils;

pub fn init() {
    println!("# rays::init");
    println!("- @TODO: init OpenCL");
    println!("- @TODO: init NVIDIA CUDA");
}
