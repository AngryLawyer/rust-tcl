use std::ffi::{CString, c_str_to_bytes};
use std::slice;

use ll::*;
use tcl::TclEnvironment;

pub struct Object <'env> {
    _env: &'env TclEnvironment,
    raw: *mut Tcl_Obj
}

impl <'env> Object <'env> {

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

    pub fn new_string(env: &'env TclEnvironment, val: &str) -> Object<'env> {
        let buf = CString::from_slice(val.as_bytes()).as_ptr();
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewStringObj(buf, val.len() as i32);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

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

    pub fn set_boolean(&mut self, val: bool) {
        unsafe {
            Tcl_SetBooleanObj(self.raw, if val { 1 } else { 0 });
        }
    }

    pub fn set_integer(&mut self, val: i32) {
        unsafe {
            Tcl_SetIntObj(self.raw, val);
        }
    }

    pub fn set_long(&mut self, val: i64) {
        unsafe {
            Tcl_SetLongObj(self.raw, val);
        }
    }

    //TODO: WideInt
    //TODO: BigNum

    pub fn set_double(&mut self, val: f64) {
        unsafe {
            Tcl_SetDoubleObj(self.raw, val);
        }
    }

    pub fn set_string(&mut self, val: &str) {
        let buf = CString::from_slice(val.as_bytes()).as_ptr();
        unsafe {
            Tcl_SetStringObj(self.raw, buf, val.len() as i32);
        }
    }

    pub fn set_byte_array(&mut self, val: &[u8]) {
        unsafe {
            Tcl_SetByteArrayObj(self.raw, val.as_ptr(), val.len() as i32);
        }
    }

    // Getters

    pub unsafe fn raw(&self) -> *mut Tcl_Obj {
        self.raw
    }

    pub fn get_string(&self) -> String {
        unsafe {
            let mut raw_string_length = 0;
            let raw_string_ptr = Tcl_GetStringFromObj(self.raw, &mut raw_string_length);
            String::from_utf8_lossy(c_str_to_bytes(&(raw_string_ptr as *const i8))).to_string()
        }
    }

    pub fn get_byte_array(&self) -> Vec<u8> {
        unsafe {
            let mut raw_length = 0;
            let raw_vec_ptr = Tcl_GetByteArrayFromObj(self.raw, &mut raw_length);
            slice::from_raw_parts(raw_vec_ptr, raw_length as usize).to_vec()
        }
    }

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
