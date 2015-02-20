use std::mem;
use tcl::*;

pub const TCL_NO_EVAL: u32	= 0x010000;
pub const TCL_EVAL_GLOBAL: u32	= 0x020000;
pub const TCL_EVAL_DIRECT: u32 = 0x040000;
pub const TCL_EVAL_INVOKE: u32 = 0x080000;
pub const TCL_CANCEL_UNWIND: u32 = 0x100000;
pub const TCL_EVAL_NOERR: u32 = 0x200000;

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
