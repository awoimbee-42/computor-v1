pub mod real;
pub use real::Real;

use crate::operators::Operator;

// TO DO //
#[derive(Debug)]
pub struct Expr {
    formula: Vec<Token>,
    equals: Option<Vec<Token>>, // todo: greater than, less than
}
impl Expr {
    pub fn new() -> Self {
        Self {
            formula: Vec::new(),
            equals: None,
        }
    }
    pub fn inner_mut(&mut self) -> &mut Vec<Token> {
        &mut self.formula
    }
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        for v in &self.formula {
            write!(f, "{}", v)?;
        }
        write!(f, ")")?;
        if let Some(equals) = &self.equals {
            write!(f, " = ")?;
            for v in equals {
                write!(f, "{}", v)?;
            }
        }
        Ok(())
    }
}
// //

#[derive(Debug)]
pub struct Group(Vec<Token>);
impl Group {
    pub fn new() -> Self {
        Group(Vec::new())
    }
    pub fn inner_mut(&mut self) -> &mut Vec<Token> {
        &mut self.0
    }
}
impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        for v in &self.0 {
            write!(f, "{}", v)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum Token {
    Operator(&'static dyn Operator),
    Value(Value),
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Operator(o) => write!(f, "{}", o),
            Token::Value(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Group(Group),
    // Fun(Fun), -> functions are groups
    Var(String),
    Real(Real),
    Float(f64),
}
use std::fmt;
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Group(g) => write!(f, "{}", g),
            Value::Var(v) => write!(f, "{}", v),
            Value::Real(r) => write!(f, "{}", r),
            Value::Float(f_) => write!(f, "{}", f_),
        }
    }
}

#[derive(Debug)]
pub struct Fun {
    name: String,
    vars: Vec<Value>,
    inner: Group,
}
