use clap::clap_app;
use lazy_static::lazy_static;

use log::debug;
use std::error::Error;
use std::io::prelude::*;
use std::io::{self, BufReader};

mod operators;
mod parsing;
mod types;

use types::TokenVec;

#[derive(Debug, Default)]
struct Config {
    is_tty: bool,
    // float2real: bool, // TODO
    // print_float: bool, // TODO
}

impl Config {
    pub fn new() -> Self {
        let is_tty = unsafe { libc::isatty(libc::STDIN_FILENO) == 1 };
        Config { is_tty }
    }
}

lazy_static! {
    static ref C: Config = Config::new();
}

fn read_line(stdin: &mut dyn BufRead) -> Option<String> {
    if C.is_tty {
        print!("> ");
        io::stdout().flush().unwrap();
    }
    stdin.lines().next().transpose().unwrap()
}

use std::borrow::BorrowMut;
use types::{Token, Value};
fn calculate_group(group: &mut TokenVec) {
    debug!("Calculate group: {}", group);

    // Calculate inner groups
    for token in group.inner_mut().iter_mut() {
        match token.borrow_mut() {
            Token::Value(v) => match v {
                Value::Group(g) => {
                    calculate_group(g);
                    // calculate_group(group); // TO DO
                    return;
                }
                _ => (),
            },
            _ => (),
        }
    }

}

fn calculate_line(line: String) {
    let mut tokens = parsing::parse_group(&mut line.chars()).unwrap();
    debug!("tokens:\n{:?}", tokens);
    debug!("expr: {}", tokens);
    calculate_group(&mut tokens);
}

fn main() {
    let _arg_matches = clap_app!(myapp =>
        (name: "computor-v1")
        (version: "0.0.1")
        (author: "Arthur W. <arthur.woimbee@gmail.com>")
        (about: "Calculates stuff")
    )
    .get_matches();

    env_logger::init();
    let mut stdin = BufReader::new(io::stdin());

    while let Some(mut line) = read_line(&mut stdin) {
        line = line.replace(" ", "");
        line = format!("({})", line);
        calculate_line(line);
    }
}
