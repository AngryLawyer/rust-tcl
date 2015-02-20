#![feature(std_misc, path, env, unsafe_destructor, core)]
extern crate "rust-tcl-sys" as ll;

pub use tcl::*;
pub use object::*;
pub use interpreter::*;

pub mod tcl;
pub mod object;
pub mod interpreter;
