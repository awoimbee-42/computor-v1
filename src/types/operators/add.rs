use super::tok2val;
use super::{Associativity, Operator};
use crate::types::Group;
use crate::types::Token;
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
    fn sign(&self) -> char {
        '+'
    }
    fn associativity(&self) -> Associativity {
        Associativity::Any
    }
    fn precedence(&self) -> u8 {
        1
    }
    fn operate(&self, group: &mut Group, id: usize) -> Result<usize, Box<dyn Error>> {
        // let t = group.index_mut(id - 1);
        let lft = tok2val(group[id - 1].clone());
        let rgt = tok2val(group[id + 1].clone());

        let res = lft + rgt;
        group[id] = Token::Value(res);
        group.remove(id + 1);
        group.remove(id - 1);

        Ok(id - 1)
    }
}
