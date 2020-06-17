use log::debug;
use std::error::Error;
use std::str::Chars;

use crate::operators::ALL_OPERATORS;
use crate::types::{Real, Token, TokenVec, Value};

#[derive(Debug, Clone)]
struct SyntaxError {
    index: usize,
}
impl SyntaxError {
    pub fn new(index: usize) -> Self {
        SyntaxError { index }
    }
}
use std::fmt;
impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TO DO")
    }
}

/// Returns index of closing parenthese, slice should not contain the opening one.
fn find_matching_prenthese(slice: &str) -> Result<&u8, Box<dyn Error>> {
    let mut count = 0;
    for (_i, c) in slice.as_bytes().iter().enumerate() {
        match c {
            b'(' => count += 1,
            b')' => count -= 1,
            _ => (),
        }
        if count == 0 {
            return Ok(c);
        }
    }
    Err("No matching closing parenthese".into())
}

fn parse_digits(iter: &mut Chars) -> Option<Value> {
    let i2 = iter.clone();
    let mut nb_len = 0;
    for c in i2 {
        if !matches!(c, '0'..='9' | '.' | 'e' | 'E') {
            break;
        }
        nb_len += 1;
    }
    let slice = &iter.as_str()[..nb_len];
    debug!("parse_digits called on: {}", slice);
    if nb_len != 0 {
        for _ in 1..nb_len {
            iter.next().unwrap();
        }
    }
    if let Ok(i) = slice.parse::<i64>() {
        Some(Value::Real(Real::from(i)))
    } else if let Ok(f) = slice.parse::<f64>() {
        Some(Value::Float(f))
    } else {
        None
    }
}

pub fn parse_group(iter: &mut Chars) -> Result<TokenVec, Box<dyn Error>> {
    let slice = iter.as_str();
    let end = find_matching_prenthese(slice)?;
    iter.next();
    debug!("Parse group: {}", slice);

    let mut tokens = TokenVec::new();

    while let Some(ch) = iter.as_str().chars().next() {
        debug!("ch: {}, as_str: {}", ch, iter.as_str());
        let t = match ch {
            c if c.is_ascii_digit() => parse_digits(iter)
                .map(|v| Ok(Token::Value(v)))
                .unwrap_or_else(|| Err("invalid number".into())),
            // c if c.is_ascii_alphabetic() => println!("{} is a var !", ch),
            c if ALL_OPERATORS.contains_key(&c) => {
                Ok(Token::Operator(*ALL_OPERATORS.get(&c).unwrap()))
            }
            '(' => parse_group(iter).map(|v| Token::Value(Value::Group(v))),
            _ => {
                debug!("Unhandled case !!!");
                Ok(Token::Value(Value::Float(4.)))
            }
        }?;
        debug!("ch token: {:?}", &t);
        tokens.inner_mut().push(t);
        iter.next();
        if std::ptr::eq(&iter.as_str().as_bytes()[0], end) {
            break;
        }
    }
    Ok(tokens)
}
