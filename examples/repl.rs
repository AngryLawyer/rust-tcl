#![feature(io)]

extern crate tcl;
use std::io;
use std::io::{BufRead, Write};

/**
 * A simple program that passes commands directly to the interpreter
 */
pub fn main() {

    let input = io::stdin();
    let output = io::stdout();
    let mut buf = String::new();

    // Initialize Tcl
    let env = tcl::init();
    // Get an interpreter
    let mut interp = env.interpreter();

    loop {
        {
            let mut in_lock = input.lock();
            in_lock.read_line(&mut buf).ok().expect("Couldn't read stdin!");
        }

        let result = match interp.eval(buf.as_slice(), tcl::EvalScope::Local) {
            tcl::TclResult::Error(string) => {
                string
            },
            _ => interp.string_result()
        };
        
        {
            let mut out_lock = output.lock();
            out_lock.write_all(result.as_bytes()).ok().expect("Couldn't write stdout!");
        }

    }
}
