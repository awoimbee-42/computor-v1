use clap::{value_t, App, Arg};
use lazy_static::lazy_static;
use libc;
use regex::Regex;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::error::Error;
use std::str::Chars;
use log::{debug, info, warn};

type TokenVec = Vec<Box<Token>>;

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

#[derive(Debug)]
enum Associativity {
    Left,
    Right,
    Any,
}
#[derive(Debug)]
struct Operator {
    precedence: u8,
    associativity: Associativity,
    // add Fn pointer thing reference here
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

#[derive(Debug)]
enum Token {
    Operator(&'static Operator),
    Value(Value),
}

#[derive(Debug)]
enum Value {
    Group(TokenVec),
    // Fun(Fun), -> functions are groups
    Var(String),
    Real(Real),
    Float(f64),
}

#[derive(Debug)]
struct Real {
    num: u64,
    denum: u64,
}

#[derive(Debug)]
struct Fun {
    name: String,
    vars: Vec<Value>,
    inner: TokenVec,
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

// fn parse_value(slice: &str) -> Result<Value, SyntaxError> {
//     Err(SyntaxError::new(0))
// }
// fn parse_operator(slice: &str) -> Result<Operator, SyntaxError> {
//     // if let Some(op) = OPERATORS.get(slice.trim()) {
//     //     Ok(*op)
//     // } else {
//     //     Err(SyntaxError::new(0))
//     // }
//     Err(SyntaxError::new(0))
// }
// fn parse_unknown(slice: &str) -> Result<Token, SyntaxError> {
//     Err(SyntaxError::new(0))
// }

/// Returns index of closing parenthese, slice should not contain the opening one.
fn find_matching_prenthese(slice: &str) -> Result<usize, Box<dyn Error>> {
    debug!("find_matching_prenthese: {}", slice);
    let mut count = 0;
    for (i, c) in slice.chars().enumerate() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => (),
        }
        if count == 0 {
            return Ok(i);
        }
    }
    Err("No matching closing parenthese")?
}

fn parse_digits(iter: &mut Chars) -> Option<Value> {
    let mut i2 = iter.clone();
    let mut nb_len = 0;
    while let Some(c) = i2.next() {
        if !matches!(c, '0'..='9' | '.' | 'e' | 'E') {
            break;
        }
        nb_len += 1;
    }
    let slice = &iter.as_str()[..nb_len];
    debug!("parse_digits called on: {}", slice);
    if nb_len != 0 {for _ in 1..nb_len {iter.next().unwrap();} }
    if let Ok(i) = slice.parse() {
        Some(Value::Real(Real{num: i, denum: 1}))
    } else if let Ok(f) = slice.parse() {
        Some(Value::Float(f))
    } else {
        None
    }
}

fn parse_group(slice: &str) -> Result<TokenVec, Box<dyn Error>> {
    let end = find_matching_prenthese(slice)?;
    let slice = &slice[1..end];

    let mut tokens: TokenVec = Vec::new();

    let mut iter = slice.chars();

    // let mut 
    while let Some(ch) = iter.as_str().chars().next() {
        println!("ch: {}, as_str: {}", ch, iter.as_str());
        let t = match ch {
            c if c.is_ascii_digit() => parse_digits(&mut iter).map(|v| Ok(Token::Value(v))).unwrap_or(Err("invalid number".into())),

            // c if c.is_ascii_alphabetic() => println!("{} is a var !", ch),
            c if OPERATORS.contains_key(&c) => Ok(Token::Operator(OPERATORS.get(&c).unwrap())),

            // '-' => println!("{} is -", ch),
            // '+' => println!("{} is +", ch),
            // '*' => println!("{} is *", ch),
            // '/' => println!("{} is /", ch),
            // '^' => println!("{} is ^", ch),
            // '!' => println!("{} is !", ch),
            '(' => parse_group(iter.as_str()).map(|v| Token::Value(Value::Group(v))),
            _ => Ok(Token::Value(Value::Float(4.))),//  panic!("pute {}", ch),
        }?;
        println!("Final token: {:?}", &t);
        tokens.push(Box::from(t));
        iter.next();
    }
    Ok(tokens)
}

fn calculate_line(line: String) {
    // lazy_static! {
    //     static ref regex_operators: Regex = Regex::new(
    //         &ALL_OPERATORS
    //             .iter()
    //             .map(|op| format!("|{}", regex::escape(op)))
    //             .collect::<String>()[1..]
    //     )
    //     .unwrap();
    // }
    println!("{}", line);
    let tokens = parse_group(&line).unwrap();
    println!("tokens:\n{:?}", tokens);

    // let mut tokens: Vec<Token>;
    // {
    //     let mut start = 0;

    //     for op_match in regex_operators.find_iter(&line) {
    //         let op_start = op_match.start();
    //         let op_end = op_match.end();

    //         let value = &line[start..op_start].trim();
    //         let operator = &line[op_start..op_end].trim();
    //         let end = &line[op_end..].trim();
    //         println!("value: {}, operator: {}, end: {}", value, operator, end);

    //         // parse_value(&line[start..op_start]);
    //         // parse_operator(&line[op_start..op_end]);

    //         start = op_end;
    //     }
    //     // println!("{}", tmp);
    // }
}

fn main() {
    env_logger::init();
    let mut stdin = BufReader::new(io::stdin());

    while let Some(mut line) = read_line(&mut stdin) {
        line = line.replace(" ", "");
        line = format!("({})", line);
        // line.push(')');
        // if find_matching_prenthese(&line).unwrap() != line.len()-1 {
        //     panic!("pute");
        // }
        calculate_line(line);
    }
}
