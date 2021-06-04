use super::Getter;
use super::value::PropertyValue;

/// A generic container for values.
///
/// A Property is a container associating a vactor of values to a string name. the
/// vector of values can include items with different data types. Check
/// PropertyValue for a list of allowed types.
#[derive(Debug, Default)]
pub struct Property {
    name: String,
    values: Vec<PropertyValue>,
}

impl Property {
    /// constructs a new empty property with a given name.
    pub fn new(name: String) -> Self {
        Property { name, values: Vec::new() }
    }

    /// Constructs a new property with a give name and value.
    pub fn create(name: String, values: Vec<PropertyValue>) -> Property {
        Property { name, values }
    }
}

impl Getter<i32> for Property {
    fn get(&self) -> Option<i32> {
        self.values.first().unwrap().get()
    }
}

impl Getter<u32> for Property {
    fn get(&self) -> Option<u32> {
        self.values.first().unwrap().get()
    }
}

impl Getter<f32> for Property {
    fn get(&self) -> Option<f32> {
        self.values.first().unwrap().get()
    }
}
