use std::ffi::{CString, c_str_to_bytes};

use ll::*;

pub struct Object {
    raw: *mut Tcl_Obj
}

impl Object {

    pub fn new() -> Object {
        Object {
            raw: unsafe {
                let raw = Tcl_NewObj();
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    pub fn new_boolean(val: bool) -> Object {
        Object {
            raw: unsafe {
                let raw = Tcl_NewBooleanObj(if val { 1 } else { 0 });
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    pub fn new_integer(val: i32) -> Object {
        Object {
            raw: unsafe {
                let raw = Tcl_NewIntObj(val);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    pub fn new_long(val: i64) -> Object {
        Object {
            raw: unsafe {
                let raw = Tcl_NewLongObj(val);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    //TODO: WideInt
    //TODO: BigNum

    pub fn new_double(val: f64) -> Object {
        Object {
            raw: unsafe {
                let raw = Tcl_NewDoubleObj(val);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    pub fn new_string(val: &str) -> Object {
        let buf = CString::from_slice(val.as_bytes()).as_ptr();
        Object {
            raw: unsafe {
                let raw = Tcl_NewStringObj(buf, val.len() as i32);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    pub fn new_byte_array(val: &[u8]) -> Object {
        Object {
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

    pub fn get_string(&mut self) -> String {
        unsafe {
            let mut raw_string_length = 0;
            let raw_string_ptr = Tcl_GetStringFromObj(self.raw, &mut raw_string_length);
            String::from_utf8_lossy(c_str_to_bytes(&(raw_string_ptr as *const i8))).to_string()
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe { Tcl_DecrRefCount(self.raw) };
    }
}
