#![feature(path, unsafe_destructor, core)]

#![crate_type= "lib"]
#![crate_name= "tcl"]

extern crate "rust-tcl-sys" as ll;
#[macro_use]
extern crate bitflags;

#[doc(no_inline)]
pub use tcl::*;
#[doc(no_inline)]
pub use object::*;
#[doc(no_inline)]
pub use interpreter::*;
#[doc(no_inline)]
pub use link::*;

pub mod tcl;
pub mod object;
pub mod interpreter;
pub mod link;
