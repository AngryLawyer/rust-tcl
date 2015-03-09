extern crate tcl;
extern crate "rust-tcl-sys" as ll;

#[test]
fn new_object() {
    let env = tcl::init();
    env.object();
}

#[test]
fn clone_object() {
    let env = tcl::init();
    let obj = env.string("TEST");
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
    let obj = env.object();
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
    ($kind:ident, $getter:ident, $setter:ident, $first_value:expr, $second_value:expr) => (
        #[test]
        fn $kind() {
            let env = tcl::init();
            let mut interp = env.interpreter().unwrap();
            let mut obj = env.$kind($first_value);
            assert_eq!(interp.$getter(&obj).unwrap(), $first_value);
            obj.$setter($second_value);
            assert_eq!(interp.$getter(&obj).unwrap(), $second_value);
        }
    )
}

object_test!(boolean, get_boolean_from_object, set_boolean, true, false);
object_test!(integer, get_integer_from_object, set_integer, 1, 2);
object_test!(long, get_long_from_object, set_long, 1i64, 2i64);
object_test!(double, get_double_from_object, set_double, 1.0f64, 2.0f64);

#[test]
fn string()  {
    let env = tcl::init();
    let mut obj = env.string("HI");
    assert_eq!(obj.get_string(), "HI");
    obj.set_string("BYE");
    assert_eq!(obj.get_string(), "BYE");
}

#[test]
fn byte_array()  {
    let env = tcl::init();
    let mut obj = env.byte_array(&[1,4]);
    assert_eq!(obj.get_byte_array(), [1,4]);
    obj.set_byte_array(&[1,2]);
    assert_eq!(obj.get_byte_array(), [1,2]);
}
