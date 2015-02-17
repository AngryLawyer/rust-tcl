extern crate "rust-tcl-sys" as ll;

#[test]
fn refcounting() {
    unsafe {
        let obj = ll::Tcl_NewObj();
        assert_eq!(false, ll::Tcl_IsShared(obj) == 1);
        ll::Tcl_IncrRefCount(obj);
        assert_eq!(true, ll::Tcl_IsShared(obj) == 1);
        ll::Tcl_IncrRefCount(obj);
        ll::Tcl_DecrRefCount(obj);
        assert_eq!(true, ll::Tcl_IsShared(obj) == 1);
        ll::Tcl_DecrRefCount(obj);
    }
}
