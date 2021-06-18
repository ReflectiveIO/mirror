//! The SLG classes are defined with this namespace.
pub use edit_action::EditAction;
pub use edit_action::EditActionList;
pub use scene::Scene;

// pub mod bsdf;
// pub mod textures;
use crate::rays;

pub mod cameras;
mod edit_action;
pub mod film;
pub mod scene;

pub fn init() {
    println!("# slg::init");

    rays::init();

    println!("- @TODO: init openvdb");
    println!("- @TODO: init OpenImageIO threads");
    println!("- @TODO: clear file name resolver");
}
