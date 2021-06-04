use std::fmt;
use std::str;

#[derive(Debug, Clone)]
pub struct Blob {
    data: Vec<u8>,
    size: usize
}

impl Blob {
    pub fn new(data: Vec<u8>, size: usize) -> Blob {
        Blob{data, size}
    }

    pub fn decode(base64_data: &str) -> Blob {
        Blob::new(Vec::from(base64_data), 0)
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl fmt::Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(str::from_utf8(&self.data).unwrap())
    }
}
