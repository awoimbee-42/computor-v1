use super::{Associativity, Operator};
use crate::types::TokenVec;
use log::debug;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Add;

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " + ")
    }
}

impl Operator for Add {
    fn sign() -> char {
        '+'
    }
    fn associativity() -> Associativity {
        Associativity::Any
    }
    fn precedence() -> u8 {
        1
    }
    fn operate(group: TokenVec, id: usize) -> Result<usize, Box<dyn Error>> {
        debug!("{:?}{}", group, id);
        Err("Not implemented (yet)".into())
    }
}
