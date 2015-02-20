use std::ffi::{c_str_to_bytes, CString};
use std::path::Path;

use ll::*;
use tcl::{TclResult, TclEnvironment};
use object::Object;

bitflags!(
    flags EvalStyle: u32 {
        const NO_EVAL = TCL_NO_EVAL,
        const GLOBAL = TCL_EVAL_GLOBAL,
        const DIRECT = TCL_EVAL_DIRECT,
        const INVOKE = TCL_EVAL_INVOKE,
        const CANCEL_UNWIND = TCL_CANCEL_UNWIND,
        const NOERR = TCL_EVAL_NOERR
    }
);

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

    pub fn is_safe(&self) -> bool {
        unsafe { Tcl_IsSafe(self.raw) == 1 }
    }

    pub fn make_safe(&mut self) -> TclResult {
        let result = unsafe { Tcl_MakeSafe(self.raw) };
        TclResult::from_ll(result, self)
    }

    pub fn string_result(&self) -> String {
        unsafe {
            let string = Tcl_GetStringResult(self.raw);
            String::from_utf8_lossy(c_str_to_bytes(&string)).to_string()
        }
    }

    pub fn eval_file(&mut self, path: &Path) -> TclResult {
        let buf = CString::from_slice(path.to_string_lossy().as_bytes()).as_ptr();
        let result = unsafe {
            Tcl_EvalFile(self.raw, buf)
        };
        TclResult::from_ll(result, self)
    }

    pub fn eval(&mut self, code: &str, flags: EvalStyle) -> TclResult {
        let buf = CString::from_slice(code.as_bytes()).as_ptr();
        let result = unsafe {
            Tcl_EvalEx(self.raw, buf, code.len() as i32, flags.bits() as i32)
        };
        TclResult::from_ll(result, self)
    }

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
