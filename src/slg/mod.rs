//! The SLG classes are defined with this namespace.

pub use self::config::Config;
pub use self::edit_action::{EditAction, EditActionList};
pub use self::scene::Scene;
pub use self::session::Session;
pub use self::state::State;
use crate::rays;

pub mod engine;
pub mod film;
pub mod image_map;
pub mod light;
pub mod material;
pub mod scene;
pub mod textures;
pub mod utils;
pub mod volume;

pub mod bsdf;
pub mod cameras;
pub mod core;
pub mod sampler;
pub mod shape;

mod config;
mod edit_action;
mod session;
mod state;

pub fn init() {
    trace!("slg::init");

    rays::init();

    warn!("@TODO: init openvdb");
    warn!("@TODO: init OpenImageIO threads");
    warn!("@TODO: clear file name resolver");
}
