//! Core is the Mirror API. It can be used to create and
//! render scenes. It includes the support for advanced new features
//! like editing materials, lights, geometry, interactive rendering and more.
//!

pub use camera::CameraTrait;
pub use camera::CameraType;
// pub use config::Config;
pub use scene::Scene;

use crate::slg;

// pub use session::Session;
// pub use state::State;
//
mod camera;
// mod config;
// pub mod film;
mod scene;
// mod session;
// mod state;

/// Initializes core API. This function is thread safe.
pub fn init() {
    println!("# core::init");
    slg::init();
}
