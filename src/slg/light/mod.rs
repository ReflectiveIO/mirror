pub use self::definitions::LightSourceDefinitions;
pub use self::distant::DistantLight;
pub use self::light::LightSourceType;
pub use self::spotlight::Spotlight;
pub use self::triangle::TriangleLight;

mod definitions;
mod distant;
mod light;
mod spotlight;
mod triangle;

pub mod strategy;
pub mod traits;
