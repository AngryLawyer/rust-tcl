extern crate tcl;

#[test]
fn create_link() {
    let env = tcl::init();
    let mut interp = env.interpreter().unwrap();
    interp.make_linked_i32("llama");

    {
        let link = interp.get_linked_i32("llama").unwrap();
        assert_eq!(link.get(), 0);
    }
    assert_eq!(interp.get_variable("llama", tcl::GetVariableScope::Standard, tcl::LeaveError::No).unwrap(), "0");


    {
        let mut link = interp.get_linked_i32("llama").unwrap();
        link.set(1);
        assert_eq!(link.get(), 1);
    }

    assert_eq!(interp.get_variable("llama", tcl::GetVariableScope::Standard, tcl::LeaveError::No).unwrap(), "1");
    interp.set_variable("llama", "2", tcl::SetVariableScope::Standard, tcl::LeaveError::No, tcl::AppendStyle::Replace);
    let link = interp.get_linked_i32("llama").unwrap();
    assert_eq!(link.get(), 2);
}
