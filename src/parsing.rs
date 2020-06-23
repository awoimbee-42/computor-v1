use log::debug;
use std::error::Error;
use std::str::Chars;

use crate::types::operators::ALL_OPERATORS;
use crate::types::{Group, Num, Token, Value, Var};

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

fn parse_number(iter: &mut Chars) -> Option<Value> {
    let (num, len) = match lexical::parse_partial::<f64, _>(iter.as_str()) {
        Ok(num) => num,
        Err(_) => return None,
    };
    for _ in 1..len {
        iter.next().unwrap();
    }
    if num == (num as i64) as f64 {
        Some(Value::Num(Num::from(num as i64)))
    } else {
        Some(Value::Num(Num::from(num)))
    }
}

fn parse_var(iter: &mut Chars) -> Option<Value> {
    let slice = iter.as_str();
    let mut len = 0;
    for c in slice.as_bytes() {
        if !matches!(c, b'A'..=b'Z' | b'a'..=b'z') {
            break;
        };
        len += 1;
    }
    if len == 0 {
        return None;
    };
    for _ in 1..len {
        iter.next();
    }
    Some(Value::Var(Var::new(&slice[..len])))
}

pub fn parse_group(iter: &mut Chars) -> Result<Group, Box<dyn Error>> {
    let slice = iter.as_str();
    let end = find_matching_prenthese(slice)?;
    iter.next();
    debug!("Parse group: {}", slice);

    let mut tokens = Group::new();
    let mut will_be_value = true;

    while let Some(ch) = iter.as_str().chars().next() {
        debug!("ch: {}", ch);
        let t = if will_be_value {
            will_be_value = false;
            Token::Value(if ch == '(' {
                Value::Group(parse_group(iter)?)
            } else if let Some(v) = parse_number(iter) {
                v
            } else if let Some(v) = parse_var(iter) {
                v
            } else {
                // TODO: var, function
                Err("Not implemented")?
            })
        } else {
            will_be_value = true;
            Token::Operator(*ALL_OPERATORS.get(&ch).unwrap())
        };

        //     // c if c.is_ascii_alphabetic() => println!("{} is a var !", ch),
        debug!("char: {}, token: {1:?} ({1:})", ch, &t);
        tokens.push(t);
        iter.next();
        if std::ptr::eq(iter.as_str().as_ptr(), end) {
            break;
        }
    }
    Ok(tokens)
}
