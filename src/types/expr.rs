use super::operators::*;
use super::Num;
use super::Term;
use super::Value;
use crate::uniq_resolve;

use log::debug;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Add((Box<Self>, Term)),
    Sub((Box<Self>, Term)),
    Term(Term),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Add((v0, v1)) => write!(f, "{} + {}", v0, v1),
            Expr::Sub((v0, v1)) => write!(f, "{} - {}", v0, v1),
            Expr::Term(v) => write!(f, "{}", v),
        }
    }
}

impl<T: Into<Term>> From<T> for Expr {
    fn from(v: T) -> Self {
        Expr::Term(v.into())
    }
}

impl Expr {
    pub fn new_add<E>(e: E, t: Term) -> Self
    where
        E: Into<Box<Expr>>,
    {
        Expr::Add((e.into(), t))
    }
    pub fn new_sub<E>(e: E, t: Term) -> Self
    where
        E: Into<Box<Expr>>,
    {
        Expr::Sub((e.into(), t))
    }
}

impl super::Resolve for Expr {
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
                if let Some(mut a) = a {
                    if let Some(mut b) = b {
                        let val = match self {
                            Self::Add(_) => a.try_add(&mut b),
                            Self::Sub(_) => a.try_sub(&mut b),
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

// use super::operators::*;
use crate::types::Resolve;
use crate::types::Var;
use std::cmp;

// TODO //
impl cmp::PartialEq<Var> for Expr {
    fn eq(&self, _rhs: &Var) -> bool {
        false
    }
}
impl cmp::PartialEq<Num> for Expr {
    fn eq(&self, _rhs: &Num) -> bool {
        false
    }
}
impl cmp::PartialEq<Expr> for Expr {
    fn eq(&self, _rhs: &Expr) -> bool {
        false
    }
}
impl cmp::PartialEq<Expr> for Var {
    fn eq(&self, _rhs: &Expr) -> bool {
        false
    }
}
impl cmp::PartialEq<Expr> for Num {
    fn eq(&self, _rhs: &Expr) -> bool {
        false
    }
}
// ---- //

// lots of todo here
impl TryAdd<Var> for Expr {
    type Output = Expr;
    fn try_add(&mut self, rhs: &mut Var) -> Option<Self::Output> {
        let mut new_expr = Expr::new_add(self.clone(), Term::from(Value::from(rhs.clone())));
        new_expr.resolve();
        Some(new_expr)
    }
}
impl TryAdd<Num> for Expr {
    type Output = Expr;
    fn try_add(&mut self, rhs: &mut Num) -> Option<Self::Output> {
        // if let ExprInner::Term(t) = &self.expr {
        //     t.try_add(rhs).map(|t| Self::from(t))
        // } else {
        //     let mut new_expr = Expr::new_add(self, Term::from(Value::from(rhs)));
        //     new_expr.resolve();
        //     Some(new_expr)
        // }
        let mut new_expr = Expr::new_add(self.clone(), Term::from(Value::from(rhs.clone())));
        new_expr.resolve();
        Some(new_expr)
    }
}
impl TryAdd<Expr> for Expr {
    type Output = Expr;
    fn try_add(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        // bad

        Some(Expr::new_add(
            self.clone(),
            Term::from(Value::from(rhs.clone())),
        ))
    }
}
impl TryAdd<Expr> for Var {
    type Output = Expr;
    fn try_add(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        rhs.try_add(self)
    }
}
impl TryAdd<Expr> for Num {
    type Output = Expr;
    fn try_add(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        rhs.try_add(self)
    }
}

impl TrySub<Var> for Expr {
    type Output = Expr;
    fn try_sub(&mut self, rhs: &mut Var) -> Option<Self::Output> {
        Some(Expr::new_sub(
            self.clone(),
            Term::from(Value::from(rhs.clone())),
        ))
    }
}
impl TrySub<Num> for Expr {
    type Output = Expr;
    fn try_sub(&mut self, rhs: &mut Num) -> Option<Self::Output> {
        Some(Expr::new_sub(
            self.clone(),
            Term::from(Value::from(rhs.clone())),
        ))
    }
}
impl TrySub<Expr> for Expr {
    type Output = Expr;
    fn try_sub(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        Some(Expr::new_sub(
            self.clone(),
            Term::from(Value::from(rhs.clone())),
        ))
    }
}
impl TrySub<Expr> for Var {
    type Output = Expr;
    fn try_sub(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        rhs.try_sub(self)
    }
}
impl TrySub<Expr> for Num {
    type Output = Expr;
    fn try_sub(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        rhs.try_sub(self)
    }
}

impl TryMul<Expr> for Expr {
    type Output = Expr;
    fn try_mul(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        None
    }
}
impl TryMul<Var> for Expr {
    type Output = Expr;
    fn try_mul(&mut self, rhs: &mut Var) -> Option<Self::Output> {
        None
    }
}
impl TryMul<Num> for Expr {
    type Output = Expr;
    fn try_mul(&mut self, rhs: &mut Num) -> Option<Self::Output> {
        None
    }
}
impl TryMul<Expr> for Var {
    type Output = Expr;
    fn try_mul(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        rhs.try_mul(self)
    }
}
impl TryMul<Expr> for Num {
    type Output = Expr;
    fn try_mul(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        rhs.try_mul(self)
    }
}

impl TryDiv<Expr> for Expr {
    type Output = Expr;
    fn try_div(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        None
    }
}
impl TryDiv<Var> for Expr {
    type Output = Expr;
    fn try_div(&mut self, rhs: &mut Var) -> Option<Self::Output> {
        None
    }
}
impl TryDiv<Num> for Expr {
    type Output = Expr;
    fn try_div(&mut self, rhs: &mut Num) -> Option<Self::Output> {
        None
    }
}
impl TryDiv<Expr> for Var {
    type Output = Expr;
    fn try_div(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        rhs.try_div(self)
    }
}
impl TryDiv<Expr> for Num {
    type Output = Expr;
    fn try_div(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        rhs.try_div(self)
    }
}

impl TryPow<Expr> for Expr {
    type Output = Expr;
    fn try_pow(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        None
    }
}
impl TryPow<Var> for Expr {
    type Output = Expr;
    fn try_pow(&mut self, rhs: &mut Var) -> Option<Self::Output> {
        None
    }
}
impl TryPow<Num> for Expr {
    type Output = Expr;
    fn try_pow(&mut self, rhs: &mut Num) -> Option<Self::Output> {
        None
    }
}
impl TryPow<Expr> for Var {
    type Output = Expr;
    fn try_pow(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        rhs.try_pow(self)
    }
}
impl TryPow<Expr> for Num {
    type Output = Expr;
    fn try_pow(&mut self, rhs: &mut Expr) -> Option<Self::Output> {
        rhs.try_pow(self)
    }
}
