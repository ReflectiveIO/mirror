use std::str::FromStr;

pub use self::mapping_2d::*;
pub use self::mapping_3d::*;

mod mapping_2d;
mod mapping_3d;

pub enum RandomMappingSeedType {
    ObjectId,
    TriangleAov,
    ObjectIdOffset,
}

impl FromStr for RandomMappingSeedType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> { todo!() }
}

impl ToString for RandomMappingSeedType {
    fn to_string(&self) -> String { todo!() }
}
