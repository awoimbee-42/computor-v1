use crate::types::TokenVec;
use phf::phf_map;
use std::error::Error;
use std::fmt::Debug;

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

// I really need associated const impls to work, or const impl to get implemented :/
pub trait Operator: Debug + Sync {
    fn sign() -> char
    where
        Self: Sized;
    fn associativity() -> Associativity
    where
        Self: Sized;
    fn precedence() -> u8
    where
        Self: Sized;
    fn operate(group: TokenVec, id: usize) -> Result<usize, Box<dyn Error>>
    where
        Self: Sized;
}
// struct Op {
//     trait: &'static dyn Operator,
// }
// pub trait MyTrait {}
// pub struct MyStruct {
//     my_trait: (dyn Operator + 'static),
// }

pub static ALL_OPERATORS: phf::Map<char, &'static dyn Operator> = phf_map! {
    '+' => &Add,
    '-' => &Sub,
    '*' => &Mul,
    '/' => &Div,
    '^' => &Pow,
    // '!' => Operator::new(3, Associativity::Left),
};

#[derive(Debug)]
pub enum Associativity {
    Left,
    Right,
    Any,
}
