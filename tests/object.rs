extern crate tcl;

#[test]
fn new_object() {
    tcl::Object::new();
}

#[test]
fn clone_object() {
    assert!(false);
}

#[test]
fn is_shared() {
    assert!(false);
}

macro_rules! object_test {
    ($kind:ident, $creator:ident, $getter:ident, $setter:ident, $first_value:expr, $second_value:expr) => (
        #[test]
        fn $kind() {
            let mut obj = tcl::Object::$creator($first_value);
            assert_eq!(obj.$getter().unwrap(), $first_value);
            obj.$setter($second_value);
            assert_eq!(obj.$getter().unwrap(), $second_value);
        }
    )
}

//object_test!(boolean, new_boolean, get_boolean, set_boolean, true, false);
//object_test!(integer, new_integer, get_integer, set_integer, 1, 2);
//object_test!(long, new_long, get_long, set_long, 1i64, 2i64);
//object_test!(double, new_double, get_double, set_double, 1.0f64, 2.0f64);
//object_test!(string, new_string, get_string, set_string, "hi", "bye");
//object_test!(byte_array, new_byte_array, get_byte_array, set_byte_array, [0, 1], [1, 2]);

#[test]
fn string()  {
    let mut obj = tcl::Object::new_string("HI");
    assert_eq!(obj.get_string(), "HI");
    obj.set_string("BYE");
    assert_eq!(obj.get_string(), "BYE");
}
