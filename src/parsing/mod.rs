use log::debug;

use crate::types::{Num, Var};

mod lexer;
use lexer::{lex, LexItem};

use std::fmt;
use std::iter::Peekable;

use super::types::*;

#[derive(Debug, Clone)]
struct ParseError<'a> {
    lexed: &'a LexItem<'a>,
    msg: String,
}
impl<'a> ParseError<'a> {
    pub fn new<S>(lexed: &'a LexItem<'a>, msg: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexed,
            msg: msg.into(),
        }
    }
}
impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {} in {:?}", self.msg, self.lexed)
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
                panic!("Invalid (or function, those are not implemented)");
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
    let mut expr = Expr::from(parse_term(item)?);
    // item.next();
    while let Some(lexed) = item.next() {
        debug!("afsdfsdfsd: {:?}", lexed);
        let last_op = match lexed {
            LexItem::Op(o) => o,
            _ => unreachable!(),
        };
        if !matches!(last_op, &"+" | &"-") {
            return Err(ParseError::new(lexed, "invalid symbol"));
        };
        expr = match last_op {
            &"+" => Expr::new_add(expr, parse_term(item)?),
            &"-" => Expr::new_sub(expr, parse_term(item)?),
            _ => unreachable!(),
        };
    }
    if item.next().is_some() {
        println!("AAAAAAAAAAAAAAAA");
        panic!();
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
