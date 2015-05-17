extern crate tcl;
use std::path::Path;

#[test]
fn create_interp() {
    let env = tcl::init();
    env.interpreter().unwrap();
}
#[test]
fn interp_safety() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    assert_eq!(interp.is_safe(), false);
    assert_eq!(interp.make_safe().is_ok(), true);
    assert_eq!(interp.is_safe(), true);
}
#[test]
fn empty_string_result() {
    let env = tcl::init();
    let interp = env.interpreter().unwrap();
    assert_eq!("".to_string(), interp.string_result());
}
#[test]
fn eval_simple_file_fail() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
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
    let mut interp = env.interpreter().unwrap();
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
    let mut interp = env.interpreter().unwrap();
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
    let mut interp = env.interpreter().unwrap();
    let command = env.new_object("expr {1 + 2}");
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
    let mut interp = env.interpreter().unwrap();
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

    let mut interp = env.interpreter().unwrap();
    let mut command_list = env.new_object(());
    interp.list_append(&mut command_list, &env.new_object("expr"));
    interp.list_append(&mut command_list, &env.new_object("1+2"));

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
    let mut interp = env.interpreter().unwrap();
    match interp.expression_boolean("1 == 1") {
        Ok(result) => assert_eq!(true, result),
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_boolean_from_object() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    let expr = env.new_object("1 == 1");
    match interp.expression_boolean_from_object(&expr) {
        Ok(result) => assert_eq!(true, result),
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_double() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    match interp.expression_double("1.0 / 2.0") {
        Ok(result) => assert_eq!(0.5, result),
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_double_from_object() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    let expr = env.new_object("1.0 / 2.0");
    match interp.expression_double_from_object(&expr) {
        Ok(result) => assert_eq!(0.5, result),
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_long() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    match interp.expression_long("1 + 1") {
        Ok(result) => assert_eq!(2, result),
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_long_from_object() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    let expr = env.new_object("1 + 1");
    match interp.expression_long_from_object(&expr) {
        Ok(result) => assert_eq!(2, result),
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn expression_object_from_object() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    let expr = env.new_object("1 + 1");
    match interp.expression_object_from_object(&expr) {
        Ok(result) => {
            assert_eq!("2".to_string(), result.get::<&str>().unwrap())
        },
        Err(otherwise) => panic!("{:?}", otherwise)
    };
}

#[test]
fn expression_string() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    match interp.expression_string("1 + 1") {
        tcl::TclResult::Ok => {
            assert_eq!("2".to_string(), interp.string_result())
        },
        otherwise => panic!("{:?}", otherwise)
    }
}

#[test]
fn set_variable() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    assert_eq!("7".to_string(), interp.set_variable("llama", "7", tcl::SetVariableScope::Standard, tcl::LeaveError::No, tcl::AppendStyle::Replace));
    match interp.eval("return $llama", tcl::EvalScope::Local) {
        tcl::TclResult::Ok => {
            assert_eq!("7".to_string(), interp.string_result())
        },
        otherwise => panic!("{:?}", otherwise)
    }
}

/*
#[test]
fn set_array_variable() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    assert_eq!("7".to_string(), interp.set_array_variable("drama", "llama", "7", tcl::SetVariableScope::Standard, tcl::LeaveError::No, tcl::AppendStyle::Replace));
    match interp.eval("return $drama(llama)", tcl::EvalScope::Local) {
        tcl::TclResult::Ok => {
            assert_eq!("7".to_string(), interp.string_result())
        },
        otherwise => panic!("{:?}", otherwise)
    }
}
*/

#[test]
fn set_object_variable() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    let var_name = env.new_object("llama");
    let obj = env.new_object(7);
    let obj_out = interp.set_object_variable(&var_name, &obj, tcl::SetVariableScope::Standard, tcl::LeaveError::No, tcl::AppendStyle::Replace).unwrap();
    assert_eq!("7".to_string(), obj_out.get::<&str>().unwrap());
    match interp.eval("return $llama", tcl::EvalScope::Local) {
        tcl::TclResult::Ok => {
            assert_eq!("7".to_string(), interp.string_result())
        },
        otherwise => panic!("{:?}", otherwise)
    }
}

/*
#[test]
fn set_object_array_variable() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    let array_name = env.new_object("drama");
    let index = env.new_object("llama");
    let obj = env.new_object(7);
    let obj_out = interp.set_object_array_variable(&index, &array_name, &obj, tcl::SetVariableScope::Standard, tcl::LeaveError::No, tcl::AppendStyle::Replace);
    assert_eq!("7".to_string(), obj_out.get::<&str>().unwrap());
    match interp.eval("return $drama(llama)", tcl::EvalScope::Local) {
        tcl::TclResult::Ok => {
            assert_eq!("7".to_string(), interp.string_result())
        },
        otherwise => panic!("{:?}", otherwise)
    }
}*/

#[test]
fn get_variable() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    interp.eval("set llama 7", tcl::EvalScope::Local);
    assert_eq!("7".to_string(), interp.get_variable("llama", tcl::GetVariableScope::Standard, tcl::LeaveError::No).unwrap());
}

#[test]
fn get_object_variable() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    interp.eval("set llama 7", tcl::EvalScope::Local);
    let out = interp.get_object_variable("llama", tcl::GetVariableScope::Standard, tcl::LeaveError::No);
    assert_eq!("7".to_string(), out.unwrap().get::<&str>().unwrap());
}

#[test]
fn get_unset_variable() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    assert!(interp.get_variable("llama", tcl::GetVariableScope::Standard, tcl::LeaveError::No).is_none());
}

#[test]
fn unset_variable() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    interp.eval("set llama 7", tcl::EvalScope::Local);
    interp.unset_variable("llama", tcl::GetVariableScope::Standard, tcl::LeaveError::No);
    assert!(interp.get_variable("llama", tcl::GetVariableScope::Standard, tcl::LeaveError::No).is_none());
}
