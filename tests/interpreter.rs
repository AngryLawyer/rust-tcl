#![feature(path)]
extern crate tcl;
use std::path::Path;

#[test]
fn create_interp() {
    let env = tcl::init();
    env.interpreter();
}
#[test]
fn interp_safety() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    assert_eq!(interp.is_safe(), false);
    assert_eq!(interp.make_safe().is_ok(), true);
    assert_eq!(interp.is_safe(), true);
}
#[test]
fn empty_string_result() {
    let env = tcl::init();
    let interp = env.interpreter();
    assert_eq!("".to_string(), interp.string_result());
}
#[test]
fn eval_simple_file_fail() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    match interp.eval_file(&Path::new("HOLOLO")) {
        tcl::TclResult::Error(message) => {
            assert_eq!("couldn\'t read file \"HOLOLO\": no such file or directory".to_string(), message)
        },
        otherwise => panic!("Should have errored, instead got {:?}", otherwise)
    }
}

#[test]
fn eval_simple_file() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    match interp.eval_file(&Path::new("tests/simple-test.tcl")) {
        tcl::TclResult::Ok => {
            assert_eq!("6".to_string(), interp.string_result())
        },
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn eval_simple() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    match interp.eval("return Hello", tcl::EvalScope::Local) {
        tcl::TclResult::Ok => {
            assert_eq!("Hello".to_string(), interp.string_result())
        },
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn eval_object() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    let command = env.string("expr {1 + 2}");
    match interp.eval_object(&command, tcl::EvalScope::Local, tcl::ByteCompile::Compile) {
        tcl::TclResult::Ok => {
            assert_eq!("3".to_string(), interp.string_result())
        },
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn object_result() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    interp.eval("expr { 1 + 2 }", tcl::EvalScope::Local);
    let obj = interp.object_result();

    let result = interp.get_integer_from_object(&obj);
    match result {
        Ok(x) => assert_eq!(3, x),
        Err(s) => panic!("{}", s)
    }
}

#[test]
fn list_append() {
    let env = tcl::init();

    let mut interp = env.interpreter();
    let mut command_list = env.object();
    interp.list_append(&mut command_list, &env.string("expr"));
    interp.list_append(&mut command_list, &env.string("1+2"));

    match interp.eval_object(&command_list, tcl::EvalScope::Local, tcl::ByteCompile::Compile) {
        tcl::TclResult::Ok => {
            assert_eq!("3".to_string(), interp.string_result())
        },
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_boolean() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    match interp.expression_boolean("1 == 1") {
        Ok(result) => assert_eq!(true, result),
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_boolean_from_object() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    let expr = env.string("1 == 1");
    match interp.expression_boolean_from_object(&expr) {
        Ok(result) => assert_eq!(true, result),
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_double() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    match interp.expression_double("1 / 2") {
        Ok(result) => assert_eq!(0.5, result),
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_double_from_object() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    let expr = env.string("1 / 2");
    match interp.expression_double_from_object(&expr) {
        Ok(result) => assert_eq!(0.5, result),
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_long() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    match interp.expression_long("1 + 1") {
        Ok(result) => assert_eq!(2, result),
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_long_from_object() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    let expr = env.string("1 + 1");
    match interp.expression_long_from_object(&expr) {
        Ok(result) => assert_eq!(2, result),
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_object_from_object() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    let expr = env.string("1 + 1");
    match interp.expression_object_from_object(&expr) {
        Ok(result) => {
            assert_eq!("2".to_string(), result.get_string())
        },
        Err(otherwise) => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_string() {
    let env = tcl::init();
    let mut interp = env.interpreter();
    match interp.expression_string("1 + 1") {
        tcl::TclResult::Ok => {
            assert_eq!("2".to_string(), interp.string_result())
        },
        otherwise => panic!("{:?}", otherwise)
    }
}
