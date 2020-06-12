use super::{Associativity, Operator};
use crate::types::TokenVec;
use log::debug;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Div;

impl fmt::Display for Div {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " / ")
    }
}

impl Operator for Div {
    fn sign() -> char {
        '/'
    }
    fn associativity() -> Associativity {
        Associativity::Left
    }
    fn precedence() -> u8 {
        1
    }
    fn operate(group: TokenVec, id: usize) -> Result<usize, Box<dyn Error>> {
        debug!("{:?}{}", group, id);
        Err("Not implemented (yet)".into())
    }
}
