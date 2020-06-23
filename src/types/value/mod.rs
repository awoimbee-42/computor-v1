pub mod group;
pub mod num;
pub mod var;
pub use group::Group;
pub use num::Num;
pub use var::Var;

use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Group(Group),
    // Fun(Fun),
    Var(Var),
    Num(Num),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Group(g) => write!(f, "{}", g),
            Value::Var(v) => write!(f, "{}", v),
            Value::Num(n) => write!(f, "{}", n),
        }
    }
}

impl From<Num> for Value {
    fn from(val: Num) -> Self {
        Value::Num(val)
    }
}
impl From<Var> for Value {
    fn from(val: Var) -> Self {
        Value::Var(val)
    }
}

macro_rules! for_any_value {
    ($matched:ident, $name:ident, $what:expr) => {
        match $matched {
            // Value::Group($name) => $what,
            Value::Var($name) => $what,
            Value::Num($name) => $what,
            _ => panic!("Operations aren't implemented for this type ({})", $matched),
        }
    };
}

pub trait Pow<T> {
    type Output;

    fn pow(self, rhs: T) -> Self::Output;
}

impl std::ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        for_any_value!(self, v0, for_any_value!(rhs, v1, Value::from(v0 + v1)))
    }
}
impl std::ops::Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        for_any_value!(self, v0, for_any_value!(rhs, v1, Value::from(v0 - v1)))
    }
}
impl std::ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        for_any_value!(self, v0, for_any_value!(rhs, v1, Value::from(v0 * v1)))
    }
}
impl std::ops::Div<Value> for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        for_any_value!(self, v0, for_any_value!(rhs, v1, Value::from(v0 / v1)))
    }
}
impl Pow<Value> for Value {
    type Output = Value;

    fn pow(self, rhs: Value) -> Self::Output {
        for_any_value!(self, v0, for_any_value!(rhs, v1, Value::from(v0.pow(v1))))
    }
}
