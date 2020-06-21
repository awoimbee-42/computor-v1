use super::{Associativity, Operator};
use crate::types::Group;
use crate::types::Token;
use crate::types::Value;
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
use std::ops::IndexMut;
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
        let mut lft = if let Token::Value(v) = group[id - 1].clone() {
            v
        } else {
            unreachable!()
        };
        let mut rgt = if let Token::Value(v) = group[id + 1].clone() {
            v
        } else {
            unreachable!()
        };

        if let Value::Group(g) = &mut lft {
            g.simplify_ref();
        }
        if let Value::Group(g) = &mut rgt {
            g.simplify_ref();
        }

        // if let Value::Group(g) = lft {

        // }

        debug!("{} {{{}}} -> {} + {}", group, id, lft, rgt);
        Err("Not implemented (yet)".into())
    }
}
