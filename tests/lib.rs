extern crate tcl;

#[test]
fn create_interp() {
    tcl::Interpreter::new();
}
