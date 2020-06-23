pub mod value;
pub use value::*;

pub mod operators;
pub use operators::*;

use std::fmt;

use std::collections::HashMap;

pub struct Computor {
    vars: HashMap<String, Value>,
    funs: HashMap<String, Group>,
}

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Fun {
    name: String,
    vars: Vec<Value>,
    inner: Group,
}
