mod parsing;
mod types;

// use std::collections::HashMap;

pub struct Computor {
    // vars: HashMap<String, Value>,
    // funs: HashMap<String, Group>,
    // float2real: bool, // TODO
    // print_float: bool, // TODO
    // ...
}

impl Computor {
    pub fn new() -> Self {
        Computor {}
    }

    pub fn compute_line(&mut self, line: &str) -> String {
        parsing::parse(line);
        "".into()
    }
}
