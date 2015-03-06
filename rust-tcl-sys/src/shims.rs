use std::mem;
use tcl::*;

pub const TCL_OK: i32 = 0;
pub const TCL_ERROR: i32 = 1;
pub const TCL_RETURN: i32 = 2;
pub const TCL_BREAK: i32 = 3;
pub const TCL_CONTINUE: i32 = 4;

pub const TCL_NO_EVAL: u32	= 0x010000;
pub const TCL_EVAL_GLOBAL: u32	= 0x020000;
pub const TCL_EVAL_DIRECT: u32 = 0x040000;
pub const TCL_EVAL_INVOKE: u32 = 0x080000;
pub const TCL_CANCEL_UNWIND: u32 = 0x100000;
pub const TCL_EVAL_NOERR: u32 = 0x200000;

pub const TCL_GLOBAL_ONLY: u32 = 1;
pub const TCL_NAMESPACE_ONLY: u32 = 2;
pub const TCL_APPEND_VALUE: u32 = 4;
pub const TCL_LIST_ELEMENT: u32 = 8;
pub const TCL_TRACE_READS: u32 = 0x10;
pub const TCL_TRACE_WRITES: u32 = 0x20;
pub const TCL_TRACE_UNSETS: u32 = 0x40;
pub const TCL_TRACE_DESTROYED: u32 = 0x80;
pub const TCL_INTERP_DESTROYED: u32 = 0x100;
pub const TCL_LEAVE_ERR_MSG: u32 = 0x200;
pub const TCL_TRACE_ARRAY: u32 = 0x800;
pub const TCL_TRACE_RESULT_DYNAMIC: u32 = 0x8000;
pub const TCL_TRACE_RESULT_OBJECT: u32 = 0x10000;

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
