use std::ffi::{CStr, CString};
use std::path::Path;

use ll::*;
use tcl::{TclResult, TclEnvironment};
use object::Object;

/*bitflags!(
    flags EvalStyle: u32 {
        const NO_EVAL = TCL_NO_EVAL,
        const GLOBAL = TCL_EVAL_GLOBAL,
        const DIRECT = TCL_EVAL_DIRECT,
        const INVOKE = TCL_EVAL_INVOKE,
        const CANCEL_UNWIND = TCL_CANCEL_UNWIND,
        const NOERR = TCL_EVAL_NOERR
    }
);*/

/// Which scope to evaluate a command in 
pub enum EvalScope {
    /// Evaluate a command at the highest-possible scope
    Global,
    /// Evaluate a command at the current scope
    Local
}

/// An instance of a Tcl interpreter
pub struct Interpreter <'env> {
    _env: &'env TclEnvironment,
    raw: *mut Tcl_Interp
}

#[unsafe_destructor]
impl <'env> Drop for Interpreter <'env> {
    fn drop(&mut self) {
        unsafe { Tcl_DeleteInterp(self.raw) };
    }
}

impl <'env> Interpreter <'env> {

    /// Create a new Interpreter
    pub fn new(env: &TclEnvironment) -> Interpreter {
        Interpreter {
            _env: env,
            raw: unsafe { Tcl_CreateInterp() }
        }
    }

    pub unsafe fn raw(&mut self) -> *mut Tcl_Interp {
        self.raw
    }

    //TODO: Child interpreters - create, get, get parent, paths

    /// Check whether the interpreter has been marked as safe
    pub fn is_safe(&self) -> bool {
        unsafe { Tcl_IsSafe(self.raw) == 1 }
    }

    /// Disable 'unsafe' commands and variables in the interpreter
    pub fn make_safe(&mut self) -> TclResult {
        let result = unsafe { Tcl_MakeSafe(self.raw) };
        TclResult::from_ll(result, self)
    }

    /// Get the string result of the last run command
    pub fn string_result(&self) -> String {
        unsafe {
            let string = Tcl_GetStringResult(self.raw);
            String::from_utf8_lossy(CStr::from_ptr(string).to_bytes()).to_string()
        }
    }

    /// Get a native Tcl object from the last run command
    pub fn object_result(&self) -> Object {
        unsafe {
            let object = Tcl_GetObjResult(self.raw);
            Object::from_raw(env, object)
        }
    }

    /// Evaluate an external file of Tcl code, and store the result internally
    pub fn eval_file(&mut self, path: &Path) -> TclResult {
        let buf = CString::new(path.to_string_lossy().as_bytes()).unwrap().as_ptr();
        let result = unsafe {
            Tcl_EvalFile(self.raw, buf)
        };
        TclResult::from_ll(result, self)
    }

    /// Evaluate a string of Tcl code, and store the result internally
    pub fn eval(&mut self, code: &str, eval_scope: EvalScope) -> TclResult {
        let buf = CString::new(code.as_bytes()).unwrap().as_ptr();

        let flags = match eval_scope {
            EvalScope::Global => TCL_EVAL_GLOBAL as i32,
            EvalScope::Local => 0
        };
        let result = unsafe {
            Tcl_EvalEx(self.raw, buf, code.len() as i32, flags)
        };
        TclResult::from_ll(result, self)
    }

    /// Attempt to extract a boolean from a Tcl value
    pub fn get_boolean_from_object(&mut self, obj: &Object) -> Result<bool, String> {
        let mut output = 0i32;
        unsafe {
            if Tcl_GetBooleanFromObj(self.raw, obj.raw(), &mut output) == 0 {
                Ok(output != 0)
            } else {
                Err(self.string_result())
            }
        }
    }

    /// Attempt to extract an integer from a Tcl value
    pub fn get_integer_from_object(&mut self, obj: &Object) -> Result<i32, String> {
        let mut output = 0i32;
        unsafe {
            if Tcl_GetIntFromObj(self.raw, obj.raw(), &mut output) == 0 {
                Ok(output)
            } else {
                Err(self.string_result())
            }
        }
    }

    /// Attempt to extract a long from a Tcl value
    pub fn get_long_from_object(&mut self, obj: &Object) -> Result<i64, String> {
        let mut output = 0i64;
        unsafe {
            if Tcl_GetLongFromObj(self.raw, obj.raw(), &mut output) == 0 {
                Ok(output)
            } else {
                Err(self.string_result())
            }
        }
    }

    //TODO: WideInt
    //TODO: BigNum

    /// Attempt to extract a double from a Tcl value
    pub fn get_double_from_object(&mut self, obj: &Object) -> Result<f64, String> {
        let mut output = 0f64;
        unsafe {
            if Tcl_GetDoubleFromObj(self.raw, obj.raw(), &mut output) == 0 {
                Ok(output)
            } else {
                Err(self.string_result())
            }
        }
    }
}
