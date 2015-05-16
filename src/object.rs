use std::ffi::{CString, CStr};
use std::slice;

use ll::*;
use tcl::TclEnvironment;

/// A Tcl value
pub struct Object<'env> {
    _env: &'env TclEnvironment,
    raw: *mut Tcl_Obj
}

pub trait TclObject {
    type FromObject;
    /// Converts self into a Tcl object.
    fn into_object(self, &TclEnvironment) -> Object;
    /// Converts from a tcl object to Self::FromObject
    fn from_object(obj: &Object) -> Self::FromObject {
        unimplemented!()
    }
    /// Updates the value of a Tcl object.
    fn set_object(self, &mut Object);
}

impl TclObject for () {
    type FromObject = ();
    
    fn into_object(self, env: &TclEnvironment) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewObj();
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    fn set_object(self, _: &mut Object) {}
}

impl TclObject for i32 {
    type FromObject = i32;
    
    fn into_object(self, env: &TclEnvironment) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewIntObj(self);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    fn set_object(self, obj: &mut Object) {
        unsafe {
            Tcl_SetIntObj(obj.raw, self);
        }
    }
}

impl TclObject for bool {
    type FromObject = bool;
    
    fn into_object(self, env: &TclEnvironment) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewBooleanObj(if self { 1 } else { 0 });
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    fn set_object(self, obj: &mut Object) {
        unsafe {
            Tcl_SetBooleanObj(obj.raw, if self { 1 } else { 0 });
        }
    }
}

impl TclObject for i64 {
    type FromObject = i64;
    
    fn into_object(self, env: &TclEnvironment) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewLongObj(self);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    fn set_object(self, obj: &mut Object) {
        unsafe {
            Tcl_SetLongObj(obj.raw, self);
        }
    }
}

//TODO: WideInt
//TODO: BigNum

impl TclObject for f64 {
    type FromObject = f64;
    
    fn into_object(self, env: &TclEnvironment) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewDoubleObj(self);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    fn set_object(self, obj: &mut Object) {
        unsafe {
            Tcl_SetDoubleObj(obj.raw, self);
        }
    }
}

impl<'a> TclObject for &'a str {
    type FromObject = String;
    
    fn into_object(self, env: &TclEnvironment) -> Object {
        let buf = CString::new(self.as_bytes()).unwrap().as_ptr();
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewStringObj(buf, self.len() as i32);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    fn set_object(self, obj: &mut Object) {
        let buf = CString::new(self.as_bytes()).unwrap().as_ptr();
        unsafe {
            Tcl_SetStringObj(obj.raw, buf, self.len() as i32);
        }
    }
    
    fn from_object(obj: &Object) -> Self::FromObject {
        unsafe {
            let mut raw_string_length = 0;
            let raw_string_ptr = Tcl_GetStringFromObj(obj.raw, &mut raw_string_length);
            String::from_utf8_lossy(CStr::from_ptr(raw_string_ptr as *const i8).to_bytes()).to_string()
        }
    }
}

impl<'a> TclObject for &'a [u8] {
    type FromObject = Vec<u8>;
    
    fn into_object(self, env: &TclEnvironment) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                let raw = Tcl_NewByteArrayObj(self.as_ptr(), self.len() as i32);
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    fn set_object(self, obj: &mut Object) {
        unsafe {
            Tcl_SetByteArrayObj(obj.raw, self.as_ptr(), self.len() as i32);
        }
    }
}

impl<'env> Object<'env> {

    pub fn from_raw(env: &TclEnvironment, raw: *mut Tcl_Obj) -> Object {
        Object {
            _env: env,
            raw: unsafe {
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }

    /// Create a new Tcl value
    pub fn new<V: TclObject>(env: &TclEnvironment, val: V) -> Object {
        val.into_object(env)
    }

    // Set the contents of a Tcl object to val
    pub fn set<V: TclObject>(&mut self, val: V) {
        val.set_object(self)
    }

    // Get the contents of a Tcl object
    pub fn get<V: TclObject>(&self) -> V::FromObject {
        V::from_object(self)
    }

    pub unsafe fn raw(&self) -> *mut Tcl_Obj {
        self.raw
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

impl<'env> Drop for Object<'env> {
    fn drop(&mut self) {
        unsafe { Tcl_DecrRefCount(self.raw) };
    }
}

impl<'env> Clone for Object<'env> {

    fn clone(&self) -> Object<'env> {
        Object {
            _env: self._env,
            raw: unsafe {
                // FIXME change clone semantics. Object is like Rc:
                // Rc::clone does not clone the contents but only the pointer
                let raw = Tcl_DuplicateObj(self.raw);
                // TODO check if this incr ref count correct in this case
                Tcl_IncrRefCount(raw);
                raw
            }
        }
    }
}
