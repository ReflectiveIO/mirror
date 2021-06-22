use std::result::Result;

use config::{ConfigError, Value};
use serde::de::Deserialize;

#[derive(Debug)]
pub struct Properties {
    props: config::Config,
}

impl Properties {
    /// Creates a new empty Properties.
    pub fn new() -> Self {
        Self {
            props: config::Config::default(),
        }
    }

    /// Sets the list of Property from a text file.
    pub fn load(filename: &str) -> Self {
        let mut props = config::Config::default();
        props.merge(config::File::with_name(filename)).unwrap();

        Self { props }
    }

    pub fn get<'de, T: Deserialize<'de>>(&self, name: &str) -> Result<T, ConfigError> {
        self.props.get(name)
    }
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}
