//! The SLG classes are defined with this namespace.
pub use self::config::Config;
pub use self::edit_action::EditAction;
pub use self::edit_action::EditActionList;
pub use self::scene::Scene;
pub use self::session::Session;
pub use self::state::State;

// pub mod bsdf;
// pub mod textures;
use crate::rays;

pub mod cameras;
mod config;
mod edit_action;
pub mod engine;
pub mod film;
pub mod scene;
mod session;
mod state;

pub fn init() {
    trace!("slg::init");

    rays::init();

    warn!("@TODO: init openvdb");
    warn!("@TODO: init OpenImageIO threads");
    warn!("@TODO: clear file name resolver");
}
