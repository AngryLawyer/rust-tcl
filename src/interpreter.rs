use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;

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

/// When setting variables, which scope to look in
pub enum SetVariableScope {
    /// Set the variable at the current scope
    Standard = 0,
    /// Set the variable at global scope
    GlobalOnly = TCL_GLOBAL_ONLY as isize,
    /// Set the variable at namespace scope
    NamespaceOnly = TCL_NAMESPACE_ONLY as isize,
}

/// When getting variables, which scope to look in
pub enum GetVariableScope {
    /// Get the variable at the current scope
    Standard = 0,
    /// Get the variable at global scope, ignoring others
    GlobalOnly = TCL_GLOBAL_ONLY as isize,
}

/// When setting a variable, should failure provide an error
pub enum LeaveError {
    No = 0,
    Yes = TCL_LEAVE_ERR_MSG as isize
}

/// When setting a variable, how should we handle existing values
pub enum AppendStyle {
    /// If the variable exists, replace it
    Replace = 0,
    /// If the variable exists, append it
    Append = TCL_APPEND_VALUE as isize,
    /// Set the variable to a single-element list
    ReplaceAsList = TCL_LIST_ELEMENT as isize,
    /// If the variable exists, append a list element to it
    AppendAsList = (TCL_APPEND_VALUE | TCL_LIST_ELEMENT) as isize
}


/// An instance of a Tcl interpreter
pub struct Interpreter <'env> {
    _env: &'env TclEnvironment,
    raw: *mut Tcl_Interp
}

impl <'env> Drop for Interpreter <'env> {
    fn drop(&mut self) {
        unsafe { Tcl_DeleteInterp(self.raw) };
    }
}

impl <'env> Interpreter <'env> {

    /// Create a new Interpreter
    pub fn new(env: &TclEnvironment) -> Result<Interpreter, &str> {
        unsafe {
            let raw = Tcl_CreateInterp();
            if raw == ptr::null_mut() {
                Err("Failed to create interpreter")
            } else {
                if Tcl_Init(raw) != TCL_OK {
                    Err("Couldn't initialize interpreter")
                } else {
                    Ok(Interpreter {
                        _env: env,
                        raw: raw
                    })
                }
            }
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
        let flags = (eval_scope as i32) | (byte_compile as i32);
        let result = unsafe {
            Tcl_EvalObjEx(self.raw, code.raw(), flags)
        };
        TclResult::from_ll(result, self)
    }

    /// Attempt to extract a boolean from a Tcl value
    pub fn get_boolean_from_object(&mut self, obj: &Object) -> Result<bool, String> {
        let mut output = 0i32;
        unsafe {
            if Tcl_GetBooleanFromObj(self.raw, obj.raw(), &mut output) == TCL_OK {
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
            if Tcl_GetIntFromObj(self.raw, obj.raw(), &mut output) == TCL_OK {
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
            if Tcl_GetLongFromObj(self.raw, obj.raw(), &mut output) == TCL_OK {
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
            if Tcl_GetDoubleFromObj(self.raw, obj.raw(), &mut output) == TCL_OK {
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

    /// Get the boolean result of an expression
    pub fn expression_boolean(&mut self, expr: &str) -> Result<bool, String> {
        let mut output = 0;
        let buf = CString::new(expr.as_bytes()).unwrap().as_ptr();
        unsafe {
            if Tcl_ExprBoolean(self.raw, buf, &mut output) == TCL_OK {
                Ok(output == 1)
            } else {
                Err(self.string_result())
            }
        }
    }

    // Get the boolean result of an expression object
    pub fn expression_boolean_from_object(&mut self, expr: &Object) -> Result<bool, String> {
        let mut output = 0;
        unsafe {
            if Tcl_ExprBooleanObj(self.raw, expr.raw(), &mut output) == TCL_OK {
                Ok(output == 1)
            } else {
                Err(self.string_result())
            }
        }
    }

    /// Get the double result of an expression
    pub fn expression_double(&mut self, expr: &str) -> Result<f64, String> {
        let mut output = 0.0;
        let buf = CString::new(expr.as_bytes()).unwrap().as_ptr();
        unsafe {
            if Tcl_ExprDouble(self.raw, buf, &mut output) == TCL_OK {
                Ok(output)
            } else {
                Err(self.string_result())
            }
        }
    }

    // Get the double result of an expression object
    pub fn expression_double_from_object(&mut self, expr: &Object) -> Result<f64, String> {
        let mut output = 0.0;
        unsafe {
            if Tcl_ExprDoubleObj(self.raw, expr.raw(), &mut output) == TCL_OK {
                Ok(output)
            } else {
                Err(self.string_result())
            }
        }
    }

    /// Get the long result of an expression
    pub fn expression_long(&mut self, expr: &str) -> Result<i64, String> {
        let mut output = 0;
        let buf = CString::new(expr.as_bytes()).unwrap().as_ptr();
        unsafe {
            if Tcl_ExprLong(self.raw, buf, &mut output) == TCL_OK {
                Ok(output)
            } else {
                Err(self.string_result())
            }
        }
    }

    // Get the long result of an expression object
    pub fn expression_long_from_object(&mut self, expr: &Object) -> Result<i64, String> {
        let mut output = 0;
        unsafe {
            if Tcl_ExprLongObj(self.raw, expr.raw(), &mut output) == TCL_OK {
                Ok(output)
            } else {
                Err(self.string_result())
            }
        }
    }

    /// Get the object result of an expression object
    pub fn expression_object_from_object(&mut self, expr: &Object) -> Result<Object<'env>, String> {
        let mut output = ptr::null_mut();
        unsafe {
            if Tcl_ExprObj(self.raw, expr.raw(), &mut output) == TCL_OK {
                Ok(Object::from_raw(self._env, output))
            } else {
                Err(self.string_result())
            }
        }
    }

    /// Process an expression string and store the result
    pub fn expression_string(&mut self, expr: &str) -> TclResult {
        let buf = CString::new(expr.as_bytes()).unwrap().as_ptr();
        let result = unsafe {
            Tcl_ExprString(self.raw, buf)
        };
        TclResult::from_ll(result, self)
    }

    /// Set a simple string variable inside the interpreter
    pub fn set_variable(&mut self, var_name: &str, new_value: &str,
        scope: SetVariableScope, leave_error: LeaveError, append_style: AppendStyle) -> String {
        let flags = scope as i32 | leave_error as i32 | append_style as i32;
        let var_buf = CString::new(var_name.as_bytes()).unwrap().as_ptr();
        let val_buf = CString::new(new_value.as_bytes()).unwrap().as_ptr();

        unsafe {
            let result = Tcl_SetVar(self.raw, var_buf, val_buf, flags);
            String::from_utf8_lossy(CStr::from_ptr(result).to_bytes()).to_string()
        }
    }

    /*
    //FIXME: Tcl 8.5 seems to be using var_buf twice, instead of array_buf
    /// Set a simple string variable for an array inside the interpreter
    pub fn set_array_variable(&mut self, array_name: &str, var_name: &str, new_value: &str,
        scope: SetVariableScope, leave_error: LeaveError, append_style: AppendStyle) -> String {

        let flags = scope as i32 | leave_error as i32 | append_style as i32;
        let array_buf = CString::new(array_name.as_bytes()).unwrap().as_ptr();
        let var_buf = CString::new(var_name.as_bytes()).unwrap().as_ptr();
        let val_buf = CString::new(new_value.as_bytes()).unwrap().as_ptr();

        unsafe {
            let result = Tcl_SetVar2(self.raw, array_buf, var_buf, val_buf, flags);
            String::from_utf8_lossy(CStr::from_ptr(result).to_bytes()).to_string()
        }
    }
    */

    /// Set an object variable inside the interpreter
    pub fn set_object_variable(&mut self, var_name: &Object, new_value: &Object,
        scope: SetVariableScope, leave_error: LeaveError, append_style: AppendStyle) -> Option<Object<'env>> {

        let flags = scope as i32 | leave_error as i32 | append_style as i32;

        unsafe {
            let result = Tcl_ObjSetVar2(self.raw, var_name.raw(), ptr::null_mut(), new_value.raw(), flags);
            if result == ptr::null_mut() {
                None
            } else {
                Some(Object::from_raw(self._env, result))
            }
        }
    }

    /*
    /// Set an object variable for an array inside the interpreter
    pub fn set_object_array_variable(&mut self, array_name: &Object, index: &Object, new_value: &Object,
        scope: SetVariableScope, leave_error: LeaveError, append_style: AppendStyle) -> Object<'env> {

        let flags = scope as i32 | leave_error as i32 | append_style as i32;

        unsafe {
            let result = Tcl_ObjSetVar2(self.raw, array_name.raw(), index.raw(), new_value.raw(), flags);
            Object::from_raw(self._env, result)
        }
    }
    */

    /// Get a simple string variable inside the interpreter
    pub fn get_variable(&mut self, var_name: &str, scope: GetVariableScope, leave_error: LeaveError) -> Option<String> {
        let flags = scope as i32 | leave_error as i32;
        let var_buf = CString::new(var_name.as_bytes()).unwrap().as_ptr();

        unsafe {
            let result = Tcl_GetVar(self.raw, var_buf, flags);
            if result != ptr::null() {
                Some(String::from_utf8_lossy(CStr::from_ptr(result).to_bytes()).to_string())
            } else {
                None
            }
        }
    }

    /// Get a variable as an object
    pub fn get_object_variable(&mut self, var_name: &str, scope: GetVariableScope, leave_error: LeaveError) -> Option<Object<'env>> {
        let flags = scope as i32 | leave_error as i32;
        let var_buf = CString::new(var_name.as_bytes()).unwrap().as_ptr();

        unsafe {
            let result = Tcl_GetVar2Ex(self.raw, var_buf, ptr::null_mut(), flags);
            if result == ptr::null_mut() {
                None
            } else {
                Some(Object::from_raw(self._env, result))
            }
        }
    }

    /// Unset a variable
    pub fn unset_variable(&mut self, var_name: &str, scope: GetVariableScope, leave_error: LeaveError) -> TclResult {
        let flags = scope as i32 | leave_error as i32;
        let buf = CString::new(var_name.as_bytes()).unwrap().as_ptr();

        let result = unsafe {
            Tcl_UnsetVar(self.raw, buf, flags)
        };
        TclResult::from_ll(result, self)
    }
}
