pub use self::definitions::LightSourceDefinitions;
pub use self::distant::DistantLight;
pub use self::light::EnvLightSource;
pub use self::light::InfiniteLightSource;
pub use self::light::IntersectableLightSource;
pub use self::light::LightSource;
pub use self::light::LightSourceType;
pub use self::light::NotIntersectableLightSource;
pub use self::triangle::TriangleLight;

mod definitions;
mod distant;
mod light;
mod triangle;

pub mod strategy;
