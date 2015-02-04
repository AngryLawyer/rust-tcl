use sys::tcl as ll;

pub struct Interpreter {
    raw: *mut ll::Tcl_Interp
}

impl Drop for Interpreter {
    fn drop(&mut self) {
        unsafe { ll::Tcl_DeleteInterp(self.raw) };
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        unsafe {
            Interpreter {
                raw: ll::Tcl_CreateInterp()
            }
        }
    }
}
