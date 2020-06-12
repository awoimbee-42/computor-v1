use crate::operators::Operator;

pub type TokenVec = Vec<Box<Token>>;

#[derive(Debug)]
pub enum Token {
    Operator(&'static dyn Operator),
    Value(Value),
}

#[derive(Debug)]
pub enum Value {
    Group(TokenVec),
    // Fun(Fun), -> functions are groups
    Var(String),
    Real(Real),
    Float(f64),
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

#[derive(Debug)]
pub struct Fun {
    name: String,
    vars: Vec<Value>,
    inner: TokenVec,
}
