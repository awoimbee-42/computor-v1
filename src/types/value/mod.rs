pub mod float;
pub mod group;
pub mod real;
pub use float::Float;
pub use group::Group;
pub use real::Real;

#[derive(Debug, Clone)]
pub enum Value {
    Group(Group),
    // Fun(Fun),
    // Var(String),
    Real(Real),
    Float(Float),
}
use std::fmt;
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Group(g) => write!(f, "{}", g),
            // Value::Var(v) => write!(f, "{}", v),
            Value::Real(r) => write!(f, "{}", r),
            Value::Float(f_) => write!(f, "{}", f_),
        }
    }
}
impl From<Real> for Value {
    fn from(val: Real) -> Self {
        Value::Real(val)
    }
}
impl From<Float> for Value {
    fn from(val: Float) -> Self {
        Value::Float(val)
    }
}

macro_rules! match_fuck {
    ($matched:ident, $name:ident, $what:expr) => {
        match $matched {
            // Value::Group($name) => $what,
            // Value::Var($name) => Value::from($what),
            Value::Real($name) => Value::from($what),
            Value::Float($name) => Value::from($what),
            _ => panic!("sadd"),
        }
    };
}

impl std::ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Self::Output {
        match_fuck!(self, v0, match_fuck!(rhs, v1, v0 + v1))
    }
}
impl std::ops::Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Self::Output {
        match_fuck!(self, v0, match_fuck!(rhs, v1, v0 - v1))
    }
}
impl std::ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Self::Output {
        match_fuck!(self, v0, match_fuck!(rhs, v1, v0 * v1))
    }
}
impl std::ops::Div<Value> for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        match_fuck!(self, v0, match_fuck!(rhs, v1, v0 / v1))
    }
}
