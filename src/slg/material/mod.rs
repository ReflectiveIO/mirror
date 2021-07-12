mod definitions;
pub use self::definitions::MaterialDefinitions;

mod material;
pub use self::material::Material;

mod glass;
pub use self::glass::GlassMaterial;

mod mirror;
pub use self::mirror::MirrorMaterial;
