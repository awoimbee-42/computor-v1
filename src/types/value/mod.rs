mod num;
mod var;

pub use num::Num;
pub use var::Var;

use super::operators::*;
use super::Expr;
use crate::uniq_resolve;
use log::debug;
use std::cmp;
use std::fmt;

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
    fn resolve(&mut self) -> Option<Value> {
        debug!("resolve: {}", self);
        match self {
            Self::Num(_) => Some(self.clone()),
            Self::Expr(e) => uniq_resolve!(self, e),
            _ => {
                eprintln!("TODO!!");
                Some(self.clone())
            } // TODO
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
impl From<Expr> for Value {
    fn from(val: Expr) -> Self {
        Value::Expr(Box::new(val))
    }
}
impl From<Box<Expr>> for Value {
    fn from(val: Box<Expr>) -> Self {
        Value::Expr(val)
    }
}

macro_rules! for_any_value {
    ($matched:ident, $name:ident, $what:expr) => {
        match $matched {
            // Value::Group($name) => $what,
            Value::Var($name) => $what,
            Value::Num($name) => $what,
            Value::Expr(box $name) => $what,
            // _ => panic!("Operations aren't implemented for this type ({})", $matched), // TODO !!
        }
    };
}

impl<T: Into<Value> + Clone> cmp::PartialEq<T> for Value {
    fn eq(&self, rhs: &T) -> bool
    where
        T: Into<Value>,
    {
        let rhs_val: Value = rhs.clone().into();
        for_any_value!(self, v0, for_any_value!(rhs_val, v1, *v0 == v1))
    }
}

impl TryAdd<Value> for Value {
    type Output = Value;
    fn try_add(self, rhs: Value) -> Option<Self::Output> {
        for_any_value!(
            self,
            v0,
            for_any_value!(rhs, v1, v0.try_add(v1).map(|v| Value::from(v)))
        )
    }
}
impl TrySub<Value> for Value {
    type Output = Value;
    fn try_sub(self, rhs: Value) -> Option<Self::Output> {
        for_any_value!(
            self,
            v0,
            for_any_value!(rhs, v1, v0.try_sub(v1).map(|v| Value::from(v)))
        )
    }
}
impl TryMul<Value> for Value {
    type Output = Value;
    fn try_mul(self, rhs: Value) -> Option<Self::Output> {
        for_any_value!(
            self,
            v0,
            for_any_value!(rhs, v1, v0.try_mul(v1).map(|v| Value::from(v)))
        )
    }
}
impl TryDiv<Value> for Value {
    type Output = Value;
    fn try_div(self, rhs: Value) -> Option<Self::Output> {
        for_any_value!(
            self,
            v0,
            for_any_value!(rhs, v1, v0.try_div(v1).map(|v| Value::from(v)))
        )
    }
}
impl TryPow<Value> for Value {
    type Output = Value;
    fn try_pow(self, rhs: Value) -> Option<Self::Output> {
        for_any_value!(
            self,
            v0,
            for_any_value!(rhs, v1, v0.try_pow(v1).map(|v| Value::from(v)))
        )
    }
}
