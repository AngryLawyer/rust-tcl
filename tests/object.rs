extern crate tcl;
extern crate "rust-tcl-sys" as ll;

#[test]
fn new_object() {
    let env = tcl::init();
    env.object();
}

#[test]
fn clone_object() {
    //assert!(false);
}

#[test]
fn is_shared() {
    //assert!(false);
}

macro_rules! object_test {
    ($kind:ident, $getter:ident, $setter:ident, $first_value:expr, $second_value:expr) => (
        #[test]
        fn $kind() {
            let env = tcl::init();
            let mut interp = env.interpreter();
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
