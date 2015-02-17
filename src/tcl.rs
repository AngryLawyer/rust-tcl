use std::ffi::{CString};
use std::env::args;
use std::ptr;
use std::sync;

use ll::*;
use interpreter::Interpreter;

static INIT_TCL: sync::Once = sync::ONCE_INIT;

pub struct TclEnvironment;

pub fn init() -> TclEnvironment {
    let ptr = match args().next() {
        Some(path) => {
            CString::from_slice(path.as_bytes()).as_ptr()
        },
        None => ptr::null()
    };

    INIT_TCL.call_once(|| {
        unsafe { Tcl_FindExecutable(ptr) };
    });

    TclEnvironment
}

impl TclEnvironment {

   pub fn interpreter(&self) -> Interpreter {
       Interpreter::new(self)
   }
}

#[derive(Debug)]
pub enum TclResult {
    Ok,
    Error(String),
    Return,
    Break,
    Continue
}

impl TclResult {
    pub fn from_ll(result: i32, interpreter: &Interpreter) -> TclResult {
        match result {
            0 => TclResult::Ok,
            1 => TclResult::Error(interpreter.string_result()),
            2 => TclResult::Return,
            3 => TclResult::Break,
            4 => TclResult::Continue,
            _ => TclResult::Error("Unknown result".to_string())
        }
    }

    pub fn is_ok(&self) -> bool {
        match *self {
            TclResult::Ok => true,
            _ => false
        }
    }
}
