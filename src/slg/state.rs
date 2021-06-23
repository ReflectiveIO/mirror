use serde::{Deserialize, Serialize};

use crate::rays::utils::Archiver;

#[derive(Default, Deserialize, Serialize, PartialEq, Debug)]
pub struct State {
    tag: String, // engine tag
}

impl State {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
        }
    }

    pub fn check_engine_tag(&self, tag: &str) {
        if tag.ne(self.tag.as_str()) {
            error!(
                "Wrong engine type in a state: {} instead of {}",
                self.tag, tag
            );
        }
    }

    pub fn get_engine_tag(&self) -> &String {
        &self.tag
    }

    pub fn save(&self, filename: &str) {
        info!("Saving render state: {}", filename);

        let mut archiver = Archiver::open(filename);
        if let Err(err) = archiver.serialize(self) {
            error!("Error while saving serialized render state: {}", filename);
        }
        archiver.flush();

        info!("Render state saved: {} bytes", archiver.stats.writes);
    }

    pub fn load(filename: &str) -> Option<State> {
        info!("Loading serialized render state: {}", filename);

        let mut archiver = Archiver::open(filename);
        let result = archiver.deserialize();

        match result {
            Ok(state) => Some(state),
            Err(err) => {
                error!("Error while loading serialized render state: {}", filename);
                None
            }
        }
    }
}
