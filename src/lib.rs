mod parsing;
mod types;

use std::collections::HashMap;
use types::Expr;
use types::Num;

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
        parsing::parse(line);

        "".into()
    }
}
