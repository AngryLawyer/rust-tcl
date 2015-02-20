#![feature(std_misc, path, env, unsafe_destructor, core, hash)]
extern crate "rust-tcl-sys" as ll;
#[macro_use]
extern crate bitflags;

pub use tcl::*;
pub use object::*;
pub use interpreter::*;

pub mod tcl;
pub mod object;
pub mod interpreter;
