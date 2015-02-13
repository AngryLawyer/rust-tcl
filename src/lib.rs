#![feature(std_misc, path)]
extern crate "rust-tcl-sys" as ll;

pub use tcl::*;
pub use object::*;

pub mod tcl;
pub mod object;
