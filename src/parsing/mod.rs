use log::debug;
use std::error::Error;

use crate::types::{Num, Var};

mod lexer;
use lexer::{lex, LexItem};

use std::fmt;
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub enum Expr {
    Add((Box<Expr>, Term)),
    Sub((Box<Expr>, Term)),
    Term(Term),
}
#[derive(Debug, Clone)]
pub enum Term {
    Mul((Box<Term>, Factor)),
    Div((Box<Term>, Factor)),
    Factor(Factor),
}
#[derive(Debug, Clone)]
pub enum Factor {
    Pow((Value, Box<Factor>)),
    Value(Value),
}
#[derive(Debug, Clone)]
pub enum Value {
    Num(Num),
    Var(Var),
    // Fun(Fun),
    Expr(Box<Expr>),
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Add((v0, v1)) => write!(f, "{} + {}", v0, v1),
            Expr::Sub((v0, v1)) => write!(f, "{} - {}", v0, v1),
            Expr::Term(v) => write!(f, "{}", v),
        }
    }
}
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Mul((v0, v1)) => write!(f, "{} * {}", v0, v1),
            Term::Div((v0, v1)) => write!(f, "{} / {}", v0, v1),
            Term::Factor(v) => write!(f, "{}", v),
        }
    }
}
impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Factor::Pow((v0, v1)) => write!(f, "{}^{}", v0, v1),
            Factor::Value(v) => write!(f, "{}", v),
        }
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Num(v) => write!(f, "{}", v),
            Value::Var(v) => write!(f, "{}", v),
            Value::Expr(v) => write!(f, "({})", v),
        }
    }
}

#[derive(Debug, Clone)]
struct ParseError<'a> {
    lexed: &'a [LexItem<'a>],
    idx: usize,
    msg: String,
}
impl<'a> ParseError<'a> {
    pub fn new<S>(lexed: &'a [LexItem<'a>], idx: usize, msg: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexed,
            idx,
            msg: msg.into(),
        }
    }
}
impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {} in {:?}", self.msg, self.lexed[self.idx])
    }
}

//////////////////////////////////////////////////////////////////////////////////

fn parse_value<'a, I>(item: &mut Peekable<I>) -> Result<Value, ParseError<'a>>
where
    I: Iterator<Item = &'a LexItem<'a>>,
{
    debug!("parse_value: {:?}", item.peek());
    match item.next() {
        Some(LexItem::Paren(p)) => {
            // item.next();
            Ok(Value::Expr(Box::new(parse_expr(&mut p.iter().peekable())?)))
        }
        Some(LexItem::Val(v)) => {
            if let Some(v) = parse_number(v) {
                Ok(Value::Num(v))
            } else if let Some(v) = parse_var(v) {
                Ok(Value::Var(v))
            } else {
                // TODO: function
                panic!("Not impl");
            }
        }
        _ => unreachable!(),
    }
}
fn parse_factor<'a, I>(item: &mut Peekable<I>) -> Result<Factor, ParseError<'a>>
where
    I: Iterator<Item = &'a LexItem<'a>> + Clone,
{
    debug!("parse_factor: {:?}", item.peek());
    let primary = parse_value(item)?;
    match item.peek() {
        Some(LexItem::Op(o)) if o == &"^" => {
            item.next();
            Ok(Factor::Pow((primary, Box::new(parse_factor(item)?))))
        }
        _ => Ok(Factor::Value(primary)),
    }
}
fn parse_term<'a, I>(item: &mut Peekable<I>) -> Result<Term, ParseError<'a>>
where
    I: Iterator<Item = &'a LexItem<'a>> + Clone,
{
    debug!("parse_term: {:?}", item.peek());
    let mut term = Term::Factor(parse_factor(item)?);
    debug!("dbg parse_term: {:?}", item.peek());
    // item.next();
    while let Some(lexed) = item.peek() {
        debug!("aaaaaaaaaa: {:?}", lexed);
        let last_op = match lexed {
            LexItem::Op(o) => o,
            _ => unreachable!(),
        };
        if !matches!(last_op, &"*" | &"/") {
            break;
        };
        item.next();
        term = match last_op {
            &"*" => Term::Mul((Box::new(term), parse_factor(item)?)),
            &"/" => Term::Div((Box::new(term), parse_factor(item)?)),
            _ => unreachable!(),
        };
    }
    Ok(term)
}
fn parse_expr<'a, I>(item: &mut Peekable<I>) -> Result<Expr, ParseError<'a>>
where
    I: Iterator<Item = &'a LexItem<'a>> + Clone,
{
    debug!("parse_expr: {:?}", item.peek());
    let mut expr = Expr::Term(parse_term(item)?);
    // item.next();
    while let Some(lexed) = item.next() {
        debug!("afsdfsdfsd: {:?}", lexed);
        let last_op = match lexed {
            LexItem::Op(o) => o,
            _ => unreachable!(),
        };
        if !matches!(last_op, &"+" | &"-") {
            break;
        };
        expr = match last_op {
            &"+" => Expr::Add((Box::new(expr), parse_term(item)?)),
            &"-" => Expr::Sub((Box::new(expr), parse_term(item)?)),
            _ => unreachable!(),
        };
    }
    Ok(expr)
}

//////////////////////////////////////////////////////////////////////////////////

pub fn parse(input: &str) -> Expr {
    debug!("Parse: {}", input);
    let lexed = lex(input);
    debug!("Lexed: {:?}", lexed);
    let parsed = parse_expr(&mut lexed.iter().peekable()).unwrap();
    debug!("Parsed: {:?}", parsed);
    parsed
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

fn parse_number(txt: &str) -> Option<Num> {
    let num = match txt.parse::<f64>() {
        Ok(num) => num,
        Err(_) => return None,
    };
    if num == (num as i64) as f64 {
        Some(Num::from(num as i64))
    } else {
        Some(Num::from(num))
    }
}

fn parse_var(txt: &str) -> Option<Var> {
    let mut len = 0;
    for c in txt.as_bytes() {
        if !matches!(c, b'A'..=b'Z' | b'a'..=b'z') {
            break;
        };
        len += 1;
    }
    if len == 0 {
        return None;
    };
    Some(Var::new(&txt[..len]))
}
