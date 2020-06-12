use clap::clap_app;
use lazy_static::lazy_static;

use log::debug;
use std::error::Error;
use std::io::prelude::*;
use std::io::{self, BufReader};

mod operators;
mod parsing;
mod types;

#[derive(Debug, Default)]
struct Config {
    is_tty: bool,
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

fn calculate_line(line: String) {
    let tokens = parsing::parse_group(&mut line.chars()).unwrap();

    debug!("tokens:\n{:?}", tokens);
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
