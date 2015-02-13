use std::mem;
use tcl::*;

pub fn Tcl_IncrRefCount(objPtr: *mut Tcl_Obj) -> () {
    unsafe {
        let transmuted: &mut Struct_Tcl_Obj = mem::transmute(objPtr);
        transmuted.refCount += 1;
    }
}

pub fn Tcl_DecrRefCount(objPtr: *mut Tcl_Obj) -> () {
    unsafe {
        let transmuted: &mut Struct_Tcl_Obj = mem::transmute(objPtr);
        transmuted.refCount -= 1;
        if transmuted.refCount <= 0 {
            TclFreeObj(objPtr);
        }
    }
}

pub fn Tcl_IsShared(objPtr: *const Tcl_Obj) -> ::libc::c_int {
    unsafe {
        let transmuted: &Struct_Tcl_Obj = mem::transmute(objPtr);
        transmuted.refCount
    }
}
