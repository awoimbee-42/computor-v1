use crate::types::Group;
use std::error::Error;
use std::fmt::{Debug, Display};

mod add;
mod div;
mod mul;
mod pow;
mod sub;

use add::Add;
use div::Div;
use mul::Mul;
use pow::Pow;
use sub::Sub;

use crate::types::{Token, Value};

// I really need associated const impls to work, or const impl to get implemented :/
pub trait Operator: Debug + Sync + Display {
    fn sign(&self) -> char;
    fn associativity(&self) -> Associativity;
    fn precedence(&self) -> u8;
    // should operate return Option ?
    fn operate(&self, group: &mut Group, id: usize) -> Result<usize, Box<dyn Error>>;
}

#[derive(Debug)]
pub enum Associativity {
    Left,
    Right,
    Any,
}

fn tok2val(t: Token) -> Value {
    if let Token::Value(v) = t {
        if let Value::Group(g) = v {
            g.simplify()
        } else {
            v
        }
    } else {
        unreachable!()
    }
}
