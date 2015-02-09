#![feature(std_misc, path)]
extern crate "rust-tcl-sys" as sys;

pub use tcl::*;

pub mod tcl;
