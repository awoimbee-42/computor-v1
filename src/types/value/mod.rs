mod var;
mod num;

pub use var::Var;
pub use num::Num;

use std::fmt;
use log::debug;
use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Value {
    Num(Num),
    Var(Var),
    // Fun(Fun),
    Expr(Box<Expr>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Num(v) => write!(f, "{}", v),
            Value::Var(v) => write!(f, "{}", v),
            Value::Expr(v) => write!(f, "({})", v),
        }
    }
}

impl super::Resolve for Value {
	type Output = Num;

	fn resolve(&mut self) -> Option<Self::Output> {
        debug!("resolve: {}", self);
		match self {
            Self::Num(v) => Some(v.clone()),
            Self::Expr(e) => e.resolve(),
            _ => None, // TODO
		}
	}
}

pub trait Pow<T> {
    type Output;

    fn pow(self, rhs: T) -> Self::Output;
}

impl<T: Into<Num>> From<T> for Value {
    fn from(val: T) -> Self {
        Value::Num(val.into())
    }
}
impl From<Var> for Value {
    fn from(val: Var) -> Self {
        Value::Var(val)
    }
}
// impl From<Expr> for Value {
//     fn from(val: Expr) -> Self {
//         Value::Expr(Box::new(val))
//     }
// }
// impl From<Box<Expr>> for Value {
//     fn from(val: Box<Expr>) -> Self {
//         Value::Expr(val)
//     }
// }


// macro_rules! for_any_value {
//     ($matched:ident, $name:ident, $what:expr) => {
//         match $matched {
//             // Value::Group($name) => $what,
//             Value::Var($name) => $what,
//             Value::Num($name) => $what,
//             _ => panic!("Operations aren't implemented for this type ({})", $matched),
//         }
//     };
// }

// impl std::ops::Add<Value> for Value {
//     type Output = Value;

//     fn add(self, rhs: Value) -> Self::Output {
//         for_any_value!(self, v0, for_any_value!(rhs, v1, Value::from(v0 + v1)))
//     }
// }
// impl std::ops::Sub<Value> for Value {
//     type Output = Value;

//     fn sub(self, rhs: Value) -> Self::Output {
//         for_any_value!(self, v0, for_any_value!(rhs, v1, Value::from(v0 - v1)))
//     }
// }
// impl std::ops::Mul<Value> for Value {
//     type Output = Value;

//     fn mul(self, rhs: Value) -> Self::Output {
//         for_any_value!(self, v0, for_any_value!(rhs, v1, Value::from(v0 * v1)))
//     }
// }
// impl std::ops::Div<Value> for Value {
//     type Output = Value;

//     fn div(self, rhs: Value) -> Self::Output {
//         for_any_value!(self, v0, for_any_value!(rhs, v1, Value::from(v0 / v1)))
//     }
// }
// impl Pow<Value> for Value {
//     type Output = Value;

//     fn pow(self, rhs: Value) -> Self::Output {
//         for_any_value!(self, v0, for_any_value!(rhs, v1, Value::from(v0.pow(v1))))
//     }
// }
