use super::tok2val;
use super::{Associativity, Operator};
use crate::types::value::Pow as lolpow;
use crate::types::Group;
use crate::types::Token;
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
        // That's how I handle right associativity...
        if let Some(Token::Operator(o)) = group.get(id + 2) {
            if o.sign() == '^' {
                return o.operate(group, id + 2);
            }
        }

        let lft = tok2val(group[id - 1].clone());
        let rgt = tok2val(group[id + 1].clone());

        let res = lft.pow(rgt);
        group[id] = Token::Value(res);
        group.remove(id + 1);
        group.remove(id - 1);

        Ok(id - 1)
    }
}
