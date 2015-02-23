use std::ffi::{CString};
use std::env::args;
use std::ptr;
use std::sync;

use ll::*;
use interpreter::Interpreter;
use object::Object;

static INIT_TCL: sync::Once = sync::ONCE_INIT;

pub struct TclEnvironment;

pub fn init() -> TclEnvironment {
    let ptr = match args().next() {
        Some(path) => {
            CString::new(path.as_bytes()).unwrap().as_ptr()
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

    pub fn object(&self) -> Object {
       Object::new(self)
    }

    pub fn boolean(&self, val: bool) -> Object {
       Object::new_boolean(self, val)
    }

    pub fn integer(&self, val: i32) -> Object {
       Object::new_integer(self, val)
    }

    pub fn long(&self, val: i64) -> Object {
       Object::new_long(self, val)
    }

    //TODO: WideInt
    //TODO: BigNum

    pub fn double(&self, val: f64) -> Object {
       Object::new_double(self, val)
    }

    pub fn string(&self, val: &str) -> Object {
        Object::new_string(self, val)
    }

    pub fn byte_array(&self, val: &[u8]) -> Object {
        Object::new_byte_array(self, val)
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
