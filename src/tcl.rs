use std::ffi::c_str_to_bytes;

use sys::tcl as ll;

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

    pub fn make_safe(&mut self) -> bool { //FIXME: Check if there's an error response type for this
        unsafe { ll::Tcl_MakeSafe(self.raw) == 0}
    }

    pub fn string_result(&self) -> String {
        unsafe {
            let string = ll::Tcl_GetStringResult(self.raw);
            String::from_utf8_lossy(c_str_to_bytes(&string)).to_string()
        }
    }
}
