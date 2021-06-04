use std::fmt::{Debug, Formatter, Result};

use super::blob::Blob;
use super::Getter;

pub union PropertyValue {
    bool_val: bool,
    i32_val: i32,
    u32_val: u32,
    f32_val: f32,
    f64_val: f64,
    i64_val: i64,
    u64_val: u64,
    str_val: &'static str,
    blob_val: &'static Blob,
}

impl Debug for PropertyValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("PropertyValue")
    }
}

impl Getter<i32> for PropertyValue {
    fn get(&self) -> Option<i32> {
        unsafe {
            Some(self.i32_val)
        }
    }
}

impl Getter<u32> for PropertyValue {
    fn get(&self) -> Option<u32> {
        unsafe {
            Some(self.u32_val)
        }
    }
}

impl Getter<f32> for PropertyValue {
    fn get(&self) -> Option<f32> {
        unsafe {
            Some(self.f32_val)
        }
    }
}

impl Getter<f64> for PropertyValue {
    fn get(&self) -> Option<f64> {
        unsafe {
            Some(self.f64_val)
        }
    }
}

impl Getter<i64> for PropertyValue {
    fn get(&self) -> Option<i64> {
        unsafe {
            Some(self.i64_val)
        }
    }
}

impl Getter<u64> for PropertyValue {
    fn get(&self) -> Option<u64> {
        unsafe {
            Some(self.u64_val)
        }
    }
}

impl Getter<&'static str> for PropertyValue {
    fn get(&self) -> Option<&'static str> {
        unsafe {
            Some(self.str_val)
        }
    }
}

impl Getter<&'static Blob> for PropertyValue {
    fn get(&self) -> Option<&'static Blob> {
        unsafe {
            Some(self.blob_val)
        }
    }
}
