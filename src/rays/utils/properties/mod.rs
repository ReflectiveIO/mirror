pub use blob::Blob;
pub use properties::Properties;
pub use property::Property;
pub use value::PropertyValue;

mod blob;
mod properties;
mod property;
mod value;

pub trait Getter<T> {
    fn get(&self) -> Option<T>;
}

