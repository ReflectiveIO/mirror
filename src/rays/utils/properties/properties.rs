use std::collections::HashMap;

use super::property::Property;

#[derive(Debug)]
pub struct Properties {
    names: Vec<String>,
    props: HashMap<String, Property>,
}

impl Properties {
    /// Creates a new empty Properties.
    pub fn new() -> Self {
        Properties {
            names: vec![],
            props: HashMap::new(),
        }
    }

    /// Sets the list of Property from a text file.
    pub fn load(filename: &str) -> Self {
        Properties::default()
    }

    /// Sets the list of Property.
    pub fn set(&mut self, props: Properties) -> Self {
        Properties::default()
    }

    /// Returns a property.
    pub fn get(&self, name: &str) -> &Property {
        match self.props.get(name) {
            Some(val) => val,
            None => panic!("not found property: {}", name),
        }
    }
}

impl Clone for Properties {
    fn clone(&self) -> Properties {
        Properties::default()
    }
}

impl Default for Properties {
    fn default() -> Self {
        Properties::new()
    }
}
