extern crate "rust-tcl-sys" as ll;

use std::ptr;

#[test]
fn refcounting() {
    unsafe {
        ll::Tcl_FindExecutable(ptr::null());
        let obj = ll::Tcl_NewObj();
        assert_eq!(false, ll::Tcl_IsShared(obj) == 1);
        ll::Tcl_IncrRefCount(obj);
        assert_eq!(false, ll::Tcl_IsShared(obj) == 1);
        ll::Tcl_IncrRefCount(obj);
        assert_eq!(true, ll::Tcl_IsShared(obj) == 1);
        ll::Tcl_DecrRefCount(obj);
        assert_eq!(false, ll::Tcl_IsShared(obj) == 1);
        ll::Tcl_DecrRefCount(obj);
    }
}
