use clap::{value_t, App, Arg};
use lazy_static::lazy_static;
use libc;
use regex::Regex;
use std::io::prelude::*;
use std::io::{self, BufReader};

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

use phf::phf_map;

enum Associativity {
    Left,
    Right,
    Any,
}
struct Operator {
    precedence: u8,
    associativity: Associativity,
}
impl Operator {
    pub const fn new(p: u8, a: Associativity) -> Self {
        Self {
            precedence: p,
            associativity: a,
        }
    }
}

static ALL_OPERATORS: [char; 5] = ['+', '-', '*', '/', '^'];
static OPERATORS: phf::Map<char, Operator> = phf_map! {
    '+' => Operator::new(1, Associativity::Any),
    '-' => Operator::new(1, Associativity::Left),
    '*' => Operator::new(1, Associativity::Any),
    '/' => Operator::new(1, Associativity::Left),
    '^' => Operator::new(1, Associativity::Right),
};

enum Token {
    Operator(Operator),
    Value(Value),
}

enum Value {
    Group(Vec<Token>),
    Fun(Fun),
    Var(String),
    Real(Real),
    Float(f64),
}

struct Group {
    members: Vec<Token>,
    start: usize,
    end: usize,
}

struct Real {
    num: u64,
    denum: u64,
}

struct Fun {
    name: String,
    vars: Vec<Value>,
    inner: Vec<Token>,
}

fn read_line(stdin: &mut dyn BufRead) -> Option<String> {
    if C.is_tty {
        print!("> ");
        io::stdout().flush().unwrap();
    }
    stdin.lines().next().transpose().unwrap()
}

#[derive(Debug, Clone)]
struct SyntaxError {
    index: usize,
}
impl SyntaxError {
    pub fn new(index: usize) -> Self {
        SyntaxError { index }
    }
    pub fn to_string(self) -> String {
        String::from("TO DO")
    }
}

fn parse_value(slice: &str) -> Result<Value, SyntaxError> {
    Err(SyntaxError::new(0))
}
fn parse_operator(slice: &str) -> Result<Operator, SyntaxError> {
    if let Some(op) = OPERATORS.get(slice.trim()) {
        Ok(*op)
    } else {
        Err(SyntaxError::new(0))
    }
}
fn parse_unknown(slice: &str) -> Result<Token, SyntaxError> {
    Err(SyntaxError::new(0))
}
fn parse_group(line: &str, start: usize, end: usize) -> Result<Value, SyntaxError> {
    let mut tokens: Vec<Token>;
    {
        let mut start = start;
        for op_i in line[start..].find(&ALL_OPERATORS[..]) {}

        for op_match in regex_operators.find_iter(&line) {
            let op_start = op_match.start();
            let op_end = op_match.end();

            let value = &line[start..op_start].trim();
            let operator = &line[op_start..op_end].trim();
            let end = &line[op_end..].trim();
            println!("value: {}, operator: {}, end: {}", value, operator, end);

            // parse_value(&line[start..op_start]);
            // parse_operator(&line[op_start..op_end]);

            start = op_end;
        }
        // println!("{}", tmp);
    }
    Err(SyntaxError::new(0))
}
fn calculate_line(line: String) {
    lazy_static! {
        static ref regex_operators: Regex = Regex::new(
            &ALL_OPERATORS
                .iter()
                .map(|op| format!("|{}", regex::escape(op)))
                .collect::<String>()[1..]
        )
        .unwrap();
    }
    println!("{}", line);
    let mut tokens: Vec<Token>;
    {
        let mut start = 0;

        for op_match in regex_operators.find_iter(&line) {
            let op_start = op_match.start();
            let op_end = op_match.end();

            let value = &line[start..op_start].trim();
            let operator = &line[op_start..op_end].trim();
            let end = &line[op_end..].trim();
            println!("value: {}, operator: {}, end: {}", value, operator, end);

            // parse_value(&line[start..op_start]);
            // parse_operator(&line[op_start..op_end]);

            start = op_end;
        }
        // println!("{}", tmp);
    }
}

fn main() {
    let mut stdin = BufReader::new(io::stdin());

    while let Some(mut line) = read_line(&mut stdin) {
        line = line.replace(" ", "");
        calculate_line(line);
    }
}
