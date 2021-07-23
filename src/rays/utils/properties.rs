use std::fmt::{Display, Formatter};
use std::result::Result;

use config::{ConfigError, Value};
use serde::de::Deserialize;

#[derive(Debug, Clone)]
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

    pub fn set<T: Into<Value>>(&mut self, name: &str, value: T) {
        if let Err(err) = self.props.set(name, value) {
            error!("set value to properties error: {}", err);
        }
    }

    pub fn has(&self, name: &str) -> bool { self.props.get_str(name).is_ok() }

    pub fn merge(&mut self, source: &Properties) { self.props.merge(source.props.clone()); }
}

impl Default for Properties {
    fn default() -> Self { Self::new() }
}

impl Display for Properties {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "Properties") }
}
