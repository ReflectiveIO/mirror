use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::{env, fs};

use bincode::ErrorKind;
use serde::{Deserialize, Serialize};

use crate::rays::utils::path_relative_from;

#[derive(Default)]
pub struct Stats {
    pub reads: usize,
    pub writes: usize,
}

pub struct Archiver {
    file: File,
    data: Vec<u8>,
    pub stats: Stats,
}

pub enum ArchiveError {
    SerializeError,
    DeserializeError,
}

/// Alias for a `Result` with the error type set to `ArchiveError`.
type Result<T> = std::result::Result<T, ArchiveError>;

impl Archiver {
    pub fn open(filename: &str) -> Self {
        // Attempt to use a relative path for the URI
        let filename = env::current_dir().unwrap().as_path().join(filename.clone());

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();

        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();

        let stats = Stats {
            reads: data.len(),
            writes: 0,
        };

        Self { file, data, stats }
    }

    pub fn serialize<T: Serialize>(&mut self, value: &T) -> Result<()> {
        let result = bincode::serialize(value);
        match result {
            Ok(encoded) => {
                self.data = encoded;
                Ok(())
            }
            Err(_) => Err(ArchiveError::SerializeError),
        }
    }

    pub fn deserialize<'de, T: Deserialize<'de>>(&'de mut self) -> Result<T> {
        let result = bincode::deserialize(&*self.data);
        match result {
            Ok(decoded) => Ok(decoded),
            Err(_) => Err(ArchiveError::DeserializeError),
        }
    }

    pub fn flush(&mut self) {
        self.file.write_all(&*self.data).unwrap();
        self.stats.writes = self.data.len();
        self.file.flush().unwrap();
    }
}
