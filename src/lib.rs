#![feature(in_band_lifetimes)]
#![allow(missing_docs)]
#![allow(unused_variables, dead_code, unused_imports)]

#[macro_use]
extern crate bitflags;
extern crate config;
#[macro_use]
extern crate downcast_rs;
#[macro_use]
extern crate log;

pub mod core;
pub mod rays;
pub mod slg;
