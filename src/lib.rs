#![feature(box_patterns)]
#![feature(peekable_next_if)]

mod parsing;
mod types;

use std::collections::HashMap;
use types::Expr;
use types::Num;
use types::Resolve;

pub struct Computor {
    vars: HashMap<String, Num>,
    funs: HashMap<String, Expr>,
    // float2real: bool, // TODO
    // print_float: bool, // TODO
    // ...
}

impl Computor {
    pub fn new() -> Self {
        Computor {
            vars: HashMap::new(),
            funs: HashMap::new(),
        }
    }

    pub fn compute_line(&mut self, line: &str) -> String {
        let mut expr = match parsing::parse(line) {
            Some(e) => e,
            None => return "".into(),
        };
        expr.resolve();
        println!("Expr: {:?}", expr);
        println!("Expr: {}", expr);
        "".into()
    }
}
