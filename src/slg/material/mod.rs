use crate::rays::color::Spectrum;
use crate::rays::geometry::Vector;

pub use self::arch_glass::ArchGlassMaterial;
pub use self::car_paint::CarPaintMaterial;
pub use self::cloth::ClothMaterial;
pub use self::definitions::MaterialDefinitions;
pub use self::disney::DisneyMaterial;
pub use self::glass::GlassMaterial;
pub use self::glass2::Glass2Material;
pub use self::glass_coating::GlassCoatingMaterial;
pub use self::glass_translucent::GlassTranslucentMaterial;
pub use self::material::Material;
pub use self::material::MaterialEmissionDLSType;
pub use self::material::MaterialTrait;
pub use self::matte::MatteMaterial;
pub use self::matte_translucent::MatteTranslucentMaterial;
pub use self::metal2::Metal2Material;
pub use self::mirror::MirrorMaterial;
pub use self::mix::MixMaterial;
pub use self::null::NullMaterial;
pub use self::rough_glass::RoughGlassMaterial;
pub use self::rough_matte::RoughMatteMaterial;
pub use self::rough_matte_translucent::RoughMatteTranslucentMaterial;
pub use self::two_sided::TwoSidedMaterial;
pub use self::velvet::VelvetMaterial;

mod arch_glass;
mod car_paint;
mod cloth;
mod definitions;
mod disney;
mod glass;
mod glass2;
mod glass_coating;
mod glass_translucent;
mod material;
mod matte;
mod matte_translucent;
mod metal2;
mod mirror;
mod mix;
mod null;
mod rough_glass;
mod rough_matte;
mod rough_matte_translucent;
mod two_sided;
mod velvet;

pub fn calc_film_color(local_fixed_dir: &Vector, film_thickness: f32, film_ior: f32) -> Spectrum {
    Spectrum::new(())
}
