use super::Num;
use super::Term;
use super::Value;
use super::operators::*;
use crate::uniq_resolve;

use log::debug;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Expr {
    name: Option<String>,
    vars: Option<HashMap<String, Num>>,
    expr: ExprInner,
}

#[derive(Debug, Clone)]
enum ExprInner {
    Add((Box<Expr>, Term)),
    Sub((Box<Expr>, Term)),
    Term(Term),
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(vars) = &self.vars {
            for (k, v) in vars {
                writeln!(f, "{}: {}", k, v)?;
            }
        }
        write!(f, "{}", self.expr)
    }
}
impl fmt::Display for ExprInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExprInner::Add((v0, v1)) => write!(f, "{} + {}", v0, v1),
            ExprInner::Sub((v0, v1)) => write!(f, "{} - {}", v0, v1),
            ExprInner::Term(v) => write!(f, "{}", v),
        }
    }
}

impl<T: Into<Term>> From<T> for Expr {
    fn from(v: T) -> Self {
        Expr {
            name: None,
            vars: None,
            expr: ExprInner::Term(v.into()),
        }
    }
}
impl<T: Into<Term>> From<T> for ExprInner {
    fn from(v: T) -> Self {
        ExprInner::Term(v.into())
    }
}

impl Expr {
    pub fn new_add<E>(e: E, t: Term) -> Self
    where
        E: Into<Box<Expr>>,
    {
        Expr {
            name: None,
            vars: None,
            expr: ExprInner::Add((e.into(), t)),
        }
    }
    pub fn new_sub<E>(e: E, t: Term) -> Self
    where
        E: Into<Box<Expr>>,
    {
        Expr {
            name: None,
            vars: None,
            expr: ExprInner::Sub((e.into(), t)),
        }
    }
}

impl super::Resolve for Expr {
    fn resolve(&mut self) -> Option<Value> {
        self.expr.resolve()
    }
}

impl super::Resolve for ExprInner {
    fn resolve(&mut self) -> Option<Value> {
        debug!("resolve: {}", self);
        match self {
            Self::Term(t) => uniq_resolve!(self, t),
            Self::Add((a, b)) | Self::Sub((a, b)) => {
                let a = a.resolve();
                let b = b.resolve();
                debug!(
                    "ADD OR SUB: a: {} b: {}",
                    a.clone().map_or("None".to_owned(), |v| format!("{}", v)),
                    b.clone().map_or("None".to_owned(), |v| format!("{}", v)),
                );
                if let Some(a) = a {
                    if let Some(b) = b {
                        let val = match self {
                            Self::Add(_) => a.try_add(b),
                            Self::Sub(_) => a.try_sub(b),
                            _ => unreachable!(),
                        };
                        if let Some(v) = &val {
                            *self = Self::from(v.clone());
                        }
                        return val;
                    }
                }
                return None;
            }
        }
    }
}
