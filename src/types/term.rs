use super::factor::Factor;
use super::operators::*;
use super::Value;
use crate::uniq_resolve;
use log::debug;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Term {
    Mul((Box<Term>, Factor)),
    Div((Box<Term>, Factor)),
    Factor(Factor),
}
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Mul((v0, v1)) => write!(f, "{} * {}", v0, v1),
            Term::Div((v0, v1)) => write!(f, "{} / {}", v0, v1),
            Term::Factor(v) => write!(f, "{}", v),
        }
    }
}

impl<T: Into<Factor>> From<T> for Term {
    fn from(val: T) -> Self {
        Term::Factor(val.into())
    }
}

impl super::Resolve for Term {
    fn resolve(&mut self) -> Option<Value> {
        debug!("resolve: {}", self);
        match self {
            Self::Factor(v) => uniq_resolve!(self, v),
            Self::Div((v0, v1)) | Self::Mul((v0, v1)) => {
                // TODO: handle mul & div of Expr
                let v0 = v0.resolve();
                let v1 = v1.resolve();
                debug!("v1: {:?} v0: {:?}", v1, v0);
                let mut v0 = match v0 {
                    Some(v) => v,
                    None => return None,
                };
                let mut v1 = match v1 {
                    Some(v) => v,
                    None => return None,
                };
                let res = match self {
                    Self::Div(_) => v0.try_div(&mut v1),
                    Self::Mul(_) => v0.try_mul(&mut v1),
                    _ => unreachable!(),
                };
                if let Some(v) = &res {
                    *self = Self::from(v.clone());
                }
                return res;
            }
        }
    }
}
