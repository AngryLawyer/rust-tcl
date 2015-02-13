use std::ffi::{CString, c_str_to_bytes};

use sys::tcl as ll;

pub struct Object {
    raw: *mut ll::Tcl_Obj
}

impl Object {

    pub fn new() -> Object {
        Object {
            raw: unsafe {
                let raw = ll::Tcl_NewObj();
                ll::Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    pub fn new_boolean(val: bool) -> Object {
        Object {
            raw: unsafe {
                let raw = ll::Tcl_NewBooleanObj(if val { 1 } else { 0 });
                ll::Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    pub fn new_integer(val: i32) -> Object {
        Object {
            raw: unsafe {
                let raw = ll::Tcl_NewIntObj(val);
                ll::Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    pub fn new_long(val: i64) -> Object {
        Object {
            raw: unsafe {
                let raw = ll::Tcl_NewLongObj(val);
                ll::Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    //TODO: WideInt
    //TODO: BigNum

    pub fn new_double(val: f64) -> Object {
        Object {
            raw: unsafe {
                let raw = ll::Tcl_NewDoubleObj(val);
                ll::Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    pub fn new_string(val: &str) -> Object {
        let buf = CString::from_slice(val.as_bytes()).as_ptr();
        Object {
            raw: unsafe {
                let raw = ll::Tcl_NewStringObj(buf, val.len() as i32);
                ll::Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    pub fn new_byte_array(val: &[u8]) -> Object {
        Object {
            raw: unsafe {
                let raw = ll::Tcl_NewByteArrayObj(val.as_ptr(), val.len() as i32);
                ll::Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    // Setters

    pub fn set_boolean(&mut self, val: bool) {
        unsafe {
            ll::Tcl_SetBooleanObj(self.raw, if val { 1 } else { 0 });
        }
    }

    pub fn set_integer(&mut self, val: i32) {
        unsafe {
            ll::Tcl_SetIntObj(self.raw, val);
        }
    }

    pub fn set_long(&mut self, val: i64) {
        unsafe {
            ll::Tcl_SetLongObj(self.raw, val);
        }
    }

    //TODO: WideInt
    //TODO: BigNum

    pub fn set_double(&mut self, val: f64) {
        unsafe {
            ll::Tcl_SetDoubleObj(self.raw, val);
        }
    }

    pub fn set_string(&mut self, val: &str) {
        let buf = CString::from_slice(val.as_bytes()).as_ptr();
        unsafe {
            ll::Tcl_SetStringObj(self.raw, buf, val.len() as i32);
        }
    }

    pub fn set_byte_array(&mut self, val: &[u8]) {
        unsafe {
            ll::Tcl_SetByteArrayObj(self.raw, val.as_ptr(), val.len() as i32);
        }
    }

    // Getters

    pub fn get_string(&mut self) -> String {
        unsafe {
            let mut raw_string_length = 0;
            let raw_string_ptr = ll::Tcl_GetStringFromObj(self.raw, &mut raw_string_length);
            String::from_utf8_lossy(c_str_to_bytes(&(raw_string_ptr as *const i8))).to_string()
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe { ll::Tcl_DecrRefCount(self.raw) };
    }
}
