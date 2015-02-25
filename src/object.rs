use std::ffi::{CString, CStr};
use std::slice;

use ll::*;
use tcl::TclEnvironment;

/// A Tcl value
pub struct Object <'env> {
    _env: &'env TclEnvironment,
    raw: *mut Tcl_Obj
}

impl <'env> Object <'env> {

    pub fn from_raw(env: &TclEnvironment, raw: *mut Tcl_Obj) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    /// Create a new untyped Tcl value
    pub fn new(env: &TclEnvironment) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewObj();
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    /// Create a new boolean Tcl value
    pub fn new_boolean(env: &TclEnvironment, val: bool) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewBooleanObj(if val { 1 } else { 0 });
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    /// Create a new integer Tcl value
    pub fn new_integer(env: &TclEnvironment, val: i32) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewIntObj(val);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    /// Create a new long Tcl value
    pub fn new_long(env: &TclEnvironment, val: i64) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewLongObj(val);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    //TODO: WideInt
    //TODO: BigNum

    /// Create a new double Tcl value
    pub fn new_double(env: &TclEnvironment, val: f64) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewDoubleObj(val);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    /// Create a new string Tcl value
    pub fn new_string(env: &'env TclEnvironment, val: &str) -> Object<'env> {
        let buf = CString::new(val.as_bytes()).unwrap().as_ptr();
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewStringObj(buf, val.len() as i32);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    /// Create a new byte array Tcl value
    pub fn new_byte_array(env: &'env TclEnvironment, val: &[u8]) -> Object<'env> {
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewByteArrayObj(val.as_ptr(), val.len() as i32);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    // Setters

    /// Set the contents of a Tcl value to a boolean
    pub fn set_boolean(&mut self, val: bool) {
        unsafe {
            Tcl_SetBooleanObj(self.raw, if val { 1 } else { 0 });
        }
    }

    /// Set the contents of a Tcl value to an integer
    pub fn set_integer(&mut self, val: i32) {
        unsafe {
            Tcl_SetIntObj(self.raw, val);
        }
    }

    /// Set the contents of a Tcl value to a long
    pub fn set_long(&mut self, val: i64) {
        unsafe {
            Tcl_SetLongObj(self.raw, val);
        }
    }

    //TODO: WideInt
    //TODO: BigNum

    /// Set the contents of a Tcl value to a double
    pub fn set_double(&mut self, val: f64) {
        unsafe {
            Tcl_SetDoubleObj(self.raw, val);
        }
    }

    /// Set the contents of a Tcl value to a string
    pub fn set_string(&mut self, val: &str) {
        let buf = CString::new(val.as_bytes()).unwrap().as_ptr();
        unsafe {
            Tcl_SetStringObj(self.raw, buf, val.len() as i32);
        }
    }

    /// Set the contents of a Tcl value to a byte array
    pub fn set_byte_array(&mut self, val: &[u8]) {
        unsafe {
            Tcl_SetByteArrayObj(self.raw, val.as_ptr(), val.len() as i32);
        }
    }

    // Getters

    pub unsafe fn raw(&self) -> *mut Tcl_Obj {
        self.raw
    }

    /// Get the string representation of a Tcl value
    pub fn get_string(&self) -> String {
        unsafe {
            let mut raw_string_length = 0;
            let raw_string_ptr = Tcl_GetStringFromObj(self.raw, &mut raw_string_length);
            String::from_utf8_lossy(CStr::from_ptr(raw_string_ptr as *const i8).to_bytes()).to_string()
        }
    }

    /// Get the byte array representation of a Tcl value
    pub fn get_byte_array(&self) -> Vec<u8> {
        unsafe {
            let mut raw_length = 0;
            let raw_vec_ptr = Tcl_GetByteArrayFromObj(self.raw, &mut raw_length);
            slice::from_raw_parts(raw_vec_ptr, raw_length as usize).to_vec()
        }
    }

    /// Is the value currently used to represent multiple variables in an interpreter
    pub fn is_shared(&self) -> bool {
        unsafe {
            Tcl_IsShared(self.raw) != 0
        }
    }
}

#[unsafe_destructor]
impl <'env> Drop for Object <'env> {
    fn drop(&mut self) {
        unsafe { Tcl_DecrRefCount(self.raw) };
    }
}

impl <'env> Clone for Object <'env> {

    fn clone(&self) -> Object<'env> {
        Object {
            _env: self._env,
            raw: unsafe {
                let raw = Tcl_DuplicateObj(self.raw);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }
}
