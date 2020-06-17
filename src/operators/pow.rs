use super::{Associativity, Operator};
use crate::types::Group;
use log::debug;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Pow;

impl fmt::Display for Pow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "^")
    }
}

impl Operator for Pow {
    fn sign(&self) -> char {
        '^'
    }
    fn associativity(&self) -> Associativity {
        Associativity::Right
    }
    fn precedence(&self) -> u8 {
        3
    }
    fn operate(&self, group: &mut Group, id: usize) -> Result<usize, Box<dyn Error>> {
        debug!("{:?}{}", group, id);
        Err("Not implemented (yet)".into())
    }
}
