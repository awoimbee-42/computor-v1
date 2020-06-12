use super::{Associativity, Operator};
use crate::types::TokenVec;
use log::debug;
use std::error::Error;

#[derive(Debug)]
pub struct Sub;
impl Operator for Sub {
    fn sign() -> char {
        '-'
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
