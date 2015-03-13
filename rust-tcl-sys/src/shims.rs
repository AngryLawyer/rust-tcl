use std::mem;
use tcl::*;

pub unsafe fn Tcl_IncrRefCount(objPtr: *mut Tcl_Obj) -> () {
    let transmuted: &mut Struct_Tcl_Obj = mem::transmute(objPtr);
    transmuted.refCount += 1;
}

pub unsafe fn Tcl_DecrRefCount(objPtr: *mut Tcl_Obj) -> () {
    let transmuted: &mut Struct_Tcl_Obj = mem::transmute(objPtr);
    transmuted.refCount -= 1;
    if transmuted.refCount <= 0 {
        TclFreeObj(transmuted);
    }
}

pub unsafe fn Tcl_IsShared(objPtr: *const Tcl_Obj) -> ::libc::c_int {
    let transmuted: &Struct_Tcl_Obj = mem::transmute(objPtr);
    if transmuted.refCount > 1 { 1 } else { 0 }
}
