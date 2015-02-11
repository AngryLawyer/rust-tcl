use std::ffi::CString;

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
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe { ll::Tcl_DecrRefCount(self.raw) };
    }
}
