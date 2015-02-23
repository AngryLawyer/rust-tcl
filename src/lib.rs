#![feature(std_misc, path, env, unsafe_destructor, core)]

#![crate_type= "lib"]

extern crate "rust-tcl-sys" as ll;
#[macro_use]
extern crate bitflags;

#[doc(no_inline)]
pub use tcl::*;
#[doc(no_inline)]
pub use object::*;
#[doc(no_inline)]
pub use interpreter::*;

pub mod tcl;
pub mod object;
pub mod interpreter;
