extern crate tcl;
extern crate rust_tcl_sys as ll;

#[test]
fn new_object() {
    let env = tcl::init();
    env.new_object(());
}

#[test]
fn clone_object() {
    let env = tcl::init();
    let obj = env.new_object("TEST");
    unsafe {
        ll::Tcl_IncrRefCount(obj.raw());
        ll::Tcl_IncrRefCount(obj.raw());
    }
    assert_eq!(true, obj.is_shared());
    let obj2 = obj.clone();
    assert_eq!(true, obj.is_shared());
    assert_eq!(obj.get_string(), "TEST");
    assert_eq!(false, obj2.is_shared());
    assert_eq!(obj2.get_string(), "TEST");

}

#[test]
fn is_shared() {

    let env = tcl::init();
    let obj = env.new_object(());
    assert_eq!(false, obj.is_shared());
    unsafe {
        ll::Tcl_IncrRefCount(obj.raw());
    }
    assert_eq!(true, obj.is_shared());
    unsafe {
        ll::Tcl_DecrRefCount(obj.raw());
    }
}

macro_rules! object_test {
    ($kind:ident, $getter:ident, $first_value:expr, $second_value:expr) => (
        #[test]
        fn $kind() {
            let env = tcl::init();
            let mut interp = env.interpreter().unwrap();
            let mut obj = env.new_object($first_value);
            assert_eq!(obj.get::<$getter>(&mut interp).unwrap(), $first_value);
            obj.set($second_value);
            assert_eq!(obj.get::<$getter>(&mut interp).unwrap(), $second_value);
        }
    )
}

object_test!(boolean, bool, true, false);
object_test!(integer, i32, 1, 2);
object_test!(long, i64, 1i64, 2i64);
object_test!(double, f64, 1.0f64, 2.0f64);

#[test]
fn string()  {
    let env = tcl::init();
    let mut obj = env.new_object("HI");
    assert_eq!(obj.get_string(), "HI");
    obj.set("BYE");
    assert_eq!(obj.get_string(), "BYE");
}

#[test]
fn byte_array()  {
    let env = tcl::init();
    let mut obj = env.new_object(&[1,4][..]);
    assert_eq!(obj.get_byte_array(), [1,4]);
    obj.set(&[1,2][..]);
    assert_eq!(obj.get_byte_array(), [1,2]);
}
