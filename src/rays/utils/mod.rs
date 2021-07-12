pub use archiver::Archiver;
pub use hair_file::HairFile;
pub use hair_file::HairFileHeader;
pub use utils::clamp;
pub use utils::path_relative_from;

pub use self::distribution::Distribution1D;

pub mod properties;

mod archiver;
mod distribution;
mod hair_file;
mod utils;
