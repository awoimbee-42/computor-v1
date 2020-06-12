use crate::operators::Operator;

#[derive(Debug)]
pub struct TokenVec(Vec<Box<Token>>);
impl TokenVec {
    pub fn new() -> Self {
        TokenVec(Vec::new())
    }
    pub fn inner_mut(&mut self) -> &mut Vec<Box<Token>> {
        &mut self.0
    }
}
impl fmt::Display for TokenVec {
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
    Group(TokenVec),
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
pub struct Real {
    num: u64,
    denum: u64,
}
impl Real {
    pub fn new(numerator: u64, denumerator: u64) -> Self {
        Real {
            num: numerator,
            denum: denumerator,
        }
    }
    /// Create a Real number from a Relative one
    pub fn from_rel(number: u64) -> Self {
        Real {
            num: number,
            denum: 1,
        }
    }
}
impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.denum {
            1 => write!(f, "{}", self.num),
            _ => write!(f, "({}/{})", self.num, self.denum),
        }
    }
}

#[derive(Debug)]
pub struct Fun {
    name: String,
    vars: Vec<Value>,
    inner: TokenVec,
}
