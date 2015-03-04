#![feature(io)]

extern crate tcl;
use std::io;
use std::io::{BufRead, Write};

/**
 * A simple program that passes commands directly to the interpreter
 */
pub fn main() {

    let input = io::stdin();

    // Initialize Tcl
    let env = tcl::init();
    // Get an interpreter
    let mut interp = env.interpreter();

    loop {
        let mut buf = String::new();
        {
            let mut in_lock = input.lock();
            in_lock.read_line(&mut buf).ok().expect("Couldn't read stdin!");
        }

        let result = interp.eval(buf.as_slice(), tcl::EvalScope::Local);

        match result {
            tcl::TclResult::Error(string) => {
                println!("Error: {}", string);
            },
            _ => println!("{}", interp.string_result())
        };

    }
}
