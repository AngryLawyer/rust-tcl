#![feature(io, core)]

extern crate tcl;
use std::io;
use std::io::BufRead;

enum QuoteType {
    None,
    Bracket,
    Quote
}

/// A class to consume input and make sure brackets/quotes match.
struct LineAccumulator {
    items: Vec<String>,
    quote_type: QuoteType,
    quote_depth: u32
}

impl LineAccumulator {

    pub fn new() -> LineAccumulator {
        LineAccumulator {
            items: vec![],
            quote_type: QuoteType::None,
            quote_depth: 0
        }
    }

    pub fn consume(&mut self, string: String) -> Option<String> {
        let mut escaped = false;

        for c in string.as_slice().chars() {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else {
                match self.quote_type {
                    QuoteType::None => {
                        if c == '"' {
                            self.quote_type = QuoteType::Quote;
                        } else if c == '{' {
                            self.quote_type = QuoteType::Bracket;
                            self.quote_depth = 1;
                        }
                    },
                    QuoteType::Bracket => {
                        if c == '{' {
                            self.quote_depth += 1;
                        } else if c == '}' {
                            self.quote_depth -= 1;
                        }

                        if self.quote_depth == 0 {
                            self.quote_type = QuoteType::None;
                        }
                    },
                    QuoteType::Quote => {
                        if c == '"' {
                            self.quote_type = QuoteType::None;
                        }
                    }
                }
            }
        };

        self.items.push(string);

        match self.quote_type {
            QuoteType::None => {
                Some(self.items.connect(" "))
            },
            _ => None
        }
    }
}

fn get_input(input: &io::Stdin) -> String {
    let mut in_lock = input.lock();
    let mut accumulator = LineAccumulator::new();

    let command;

    loop {
        let mut buf = String::new();
        in_lock.read_line(&mut buf).ok().expect("Couldn't read stdin!");

        match accumulator.consume(buf) {
            Some(string) => {
                command = string;
                break;
            },
            None => ()
        };
    };

    command
}

/**
 * A simple program that passes commands directly to the interpreter
 */
pub fn main() {

    let input = io::stdin();

    // Initialize Tcl
    let env = tcl::init();
    // Get an interpreter
    let mut interp = env.interpreter().unwrap();

    loop {
        let command = get_input(&input);
        let result = interp.eval(command.as_slice(), tcl::EvalScope::Local);

        match result {
            tcl::TclResult::Error(string) => {
                println!("Error: {}", string);
            },
            _ => println!("{}", interp.string_result())
        };

    }
}
