//! Core is the Mirror API. It can be used to create and
//! render scenes. It includes the support for advanced new features
//! like editing materials, lights, geometry, interactive rendering and more.
//!
use crate::slg;

pub use self::camera::CameraTrait;
pub use self::camera::CameraType;
pub use self::config::Config;
pub use self::film::Film;
pub use self::film::FilmChannel;
pub use self::film::FilmOutput;
pub use self::scene::Scene;
pub use self::session::Session;
pub use self::state::State;

mod camera;
mod config;
mod film;
mod scene;
mod session;
mod state;

/// Initializes core API. This function is thread safe.
pub fn init() {
    trace!("core::init");
    slg::init();
}
