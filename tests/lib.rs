#![feature(path)]
extern crate tcl;
use std::path::Path;

#[test]
fn create_interp() {
    tcl::Interpreter::new();
}
#[test]
fn interp_safety() {
    let mut interp = tcl::Interpreter::new();
    assert_eq!(interp.is_safe(), false);
    assert_eq!(interp.make_safe().is_ok(), true);
    assert_eq!(interp.is_safe(), true);
}
#[test]
fn empty_string_result() {
    let interp = tcl::Interpreter::new();
    assert_eq!("".to_string(), interp.string_result());
}
#[test]
fn eval_simple_file_fail() {
    let mut interp = tcl::Interpreter::new();
    match interp.eval_file(&Path::new("HOLOLO")) {
        tcl::TclResult::Error(message) => {
            assert_eq!("couldn\'t read file \"HOLOLO\": no such file or directory".to_string(), message)
        },
        otherwise => panic!("Should have errored, instead got {:?}", otherwise)
    }
}

#[test]
fn eval_simple_file() {
    let mut interp = tcl::Interpreter::new();
    match interp.eval_file(&Path::new("tests/simple-test.tcl")) {
        tcl::TclResult::Ok => {
            assert_eq!("6".to_string(), interp.string_result())
        },
        otherwise => panic!("Should have errored, instead got {:?}", otherwise)
    }
}
