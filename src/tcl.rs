use std::ffi::{c_str_to_bytes, CString};
use std::path::Path;

use sys::tcl as ll;

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

pub struct Interpreter {
    raw: *mut ll::Tcl_Interp
}

impl Drop for Interpreter {
    fn drop(&mut self) {
        unsafe { ll::Tcl_DeleteInterp(self.raw) };
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            raw: unsafe { ll::Tcl_CreateInterp() }
        }
    }

    //TODO: Child interpreters - create, get, get parent, paths

    pub fn is_safe(&self) -> bool {
        unsafe { ll::Tcl_IsSafe(self.raw) == 1 }
    }

    pub fn make_safe(&mut self) -> TclResult {
        let result = unsafe { ll::Tcl_MakeSafe(self.raw) };
        TclResult::from_ll(result, self)
    }

    pub fn string_result(&self) -> String {
        unsafe {
            let string = ll::Tcl_GetStringResult(self.raw);
            String::from_utf8_lossy(c_str_to_bytes(&string)).to_string()
        }
    }

    pub fn eval_file(&mut self, path: &Path) -> TclResult {
        let buf = CString::from_slice(path.to_string_lossy().as_bytes()).as_ptr();
        let result = unsafe {
            ll::Tcl_EvalFile(self.raw, buf)
        };
        TclResult::from_ll(result, self)
    }

    pub fn eval(&mut self, code: &str) -> TclResult {
        let buf = CString::from_slice(code.as_bytes()).as_ptr();
        let result = unsafe {
            ll::Tcl_Eval(self.raw, buf)
        };
        TclResult::from_ll(result, self)
    }
}
