use std::ffi::{CStr, CString};
use std::path::Path;

use ll::*;
use tcl::{TclResult, TclEnvironment};
use object::Object;

/// Which scope to evaluate a command in 
pub enum EvalScope {
    /// Evaluate a command at the current scope
    Local = 0,
    /// Evaluate a command at the highest-possible scope
    Global = TCL_EVAL_GLOBAL as isize,
}

/// Should we byte compile a command
pub enum ByteCompile {
    /// Compile the command into bytecode
    Compile = 0,
    /// Do not compile the command into bytecode
    Direct = TCL_EVAL_DIRECT as isize
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
    pub fn object_result(&self) -> Object<'env> {
        unsafe {
            let object = Tcl_GetObjResult(self.raw);
            Object::from_raw(self._env, object)
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

        let flags = eval_scope as i32;

        let result = unsafe {
            Tcl_EvalEx(self.raw, buf, code.len() as i32, flags)
        };
        TclResult::from_ll(result, self)
    }

    /// Evaluate a Tcl objecrt as code, and store the result internally
    pub fn eval_object(&mut self, code: &Object, eval_scope: EvalScope, byte_compile: ByteCompile) -> TclResult {
        let flags = (eval_scope as i32) & (byte_compile as i32);
        let result = unsafe {
            Tcl_EvalObjEx(self.raw, code.raw(), flags)
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

    /// Append an element to a list
    /// This will fail if target is shared
    pub fn list_append(&mut self, target: &mut Object, source: &Object) -> TclResult {
        let result = unsafe {
            Tcl_ListObjAppendElement(self.raw, target.raw(), source.raw())
        };
        TclResult::from_ll(result, self)
    }
}
