use super::super::operators::*;
use super::super::Term;
use super::Expr;
use super::Num;
use super::Value;
use super::Var;
use crate::types::Resolve;
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
    fn try_add(self, rhs: Var) -> Option<Self::Output> {
        let mut new_expr = Expr::new_add(self, Term::from(Value::from(rhs)));
        new_expr.resolve();
        Some(new_expr)
    }
}
impl TryAdd<Num> for Expr {
    type Output = Expr;
    fn try_add(self, rhs: Num) -> Option<Self::Output> {
        let mut new_expr = Expr::new_add(self, Term::from(Value::from(rhs)));
        new_expr.resolve();
        Some(new_expr)
    }
}
impl TryAdd<Expr> for Expr {
    type Output = Expr;
    fn try_add(self, rhs: Expr) -> Option<Self::Output> {
        Some(Expr::new_add(self, Term::from(Value::from(rhs))))
    }
}
impl TryAdd<Expr> for Var {
    type Output = Expr;
    fn try_add(self, rhs: Expr) -> Option<Self::Output> {
        rhs.try_add(self)
    }
}
impl TryAdd<Expr> for Num {
    type Output = Expr;
    fn try_add(self, rhs: Expr) -> Option<Self::Output> {
        rhs.try_add(self)
    }
}

impl TrySub<Var> for Expr {
    type Output = Expr;
    fn try_sub(self, rhs: Var) -> Option<Self::Output> {
        Some(Expr::new_sub(self, Term::from(Value::from(rhs))))
    }
}
impl TrySub<Num> for Expr {
    type Output = Expr;
    fn try_sub(self, rhs: Num) -> Option<Self::Output> {
        Some(Expr::new_sub(self, Term::from(Value::from(rhs))))
    }
}
impl TrySub<Expr> for Expr {
    type Output = Expr;
    fn try_sub(self, rhs: Expr) -> Option<Self::Output> {
        Some(Expr::new_sub(self, Term::from(Value::from(rhs))))
    }
}
impl TrySub<Expr> for Var {
    type Output = Expr;
    fn try_sub(self, rhs: Expr) -> Option<Self::Output> {
        rhs.try_sub(self)
    }
}
impl TrySub<Expr> for Num {
    type Output = Expr;
    fn try_sub(self, rhs: Expr) -> Option<Self::Output> {
        rhs.try_sub(self)
    }
}

impl TryMul<Expr> for Expr {
    type Output = Expr;
    fn try_mul(self, rhs: Expr) -> Option<Self::Output> {
        None
    }
}
impl TryMul<Var> for Expr {
    type Output = Expr;
    fn try_mul(self, rhs: Var) -> Option<Self::Output> {
        None
    }
}
impl TryMul<Num> for Expr {
    type Output = Expr;
    fn try_mul(self, rhs: Num) -> Option<Self::Output> {
        None
    }
}
impl TryMul<Expr> for Var {
    type Output = Expr;
    fn try_mul(self, rhs: Expr) -> Option<Self::Output> {
        rhs.try_mul(self)
    }
}
impl TryMul<Expr> for Num {
    type Output = Expr;
    fn try_mul(self, rhs: Expr) -> Option<Self::Output> {
        rhs.try_mul(self)
    }
}

impl TryDiv<Expr> for Expr {
    type Output = Expr;
    fn try_div(self, rhs: Expr) -> Option<Self::Output> {
        None
    }
}
impl TryDiv<Var> for Expr {
    type Output = Expr;
    fn try_div(self, rhs: Var) -> Option<Self::Output> {
        None
    }
}
impl TryDiv<Num> for Expr {
    type Output = Expr;
    fn try_div(self, rhs: Num) -> Option<Self::Output> {
        None
    }
}
impl TryDiv<Expr> for Var {
    type Output = Expr;
    fn try_div(self, rhs: Expr) -> Option<Self::Output> {
        rhs.try_div(self)
    }
}
impl TryDiv<Expr> for Num {
    type Output = Expr;
    fn try_div(self, rhs: Expr) -> Option<Self::Output> {
        rhs.try_div(self)
    }
}

impl TryPow<Expr> for Expr {
    type Output = Expr;
    fn try_pow(self, rhs: Expr) -> Option<Self::Output> {
        None
    }
}
impl TryPow<Var> for Expr {
    type Output = Expr;
    fn try_pow(self, rhs: Var) -> Option<Self::Output> {
        None
    }
}
impl TryPow<Num> for Expr {
    type Output = Expr;
    fn try_pow(self, rhs: Num) -> Option<Self::Output> {
        None
    }
}
impl TryPow<Expr> for Var {
    type Output = Expr;
    fn try_pow(self, rhs: Expr) -> Option<Self::Output> {
        rhs.try_pow(self)
    }
}
impl TryPow<Expr> for Num {
    type Output = Expr;
    fn try_pow(self, rhs: Expr) -> Option<Self::Output> {
        rhs.try_pow(self)
    }
}
