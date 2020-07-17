use super::Expr;
use super::Var;
use super::Num;
use super::Value;
use super::super::Term;
use super::super::operators::*;
use std::cmp;

impl cmp::PartialEq<Var> for Expr {
	fn eq(&self, _rhs: &Var) -> bool {
		false
	}
}

// lots of todo here
impl TryAdd<Var> for Expr {
    type Output = Expr;
    fn try_add(self, rhs: Var) -> Option<Self::Output> {
		Some(Expr::new_add(self, Term::from(Value::from(rhs))))
    }
}
impl TryAdd<Num> for Expr {
    type Output = Expr;
    fn try_add(self, rhs: Num) -> Option<Self::Output> {
		Some(Expr::new_add(self, Term::from(Value::from(rhs))))
    }
}
impl TryAdd<Expr> for Expr {
    type Output = Expr;
    fn try_add(self, rhs: Expr) -> Option<Self::Output> {
		Some(Expr::new_add(self, Term::from(Value::from(rhs))))
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

impl TryMul<Expr> for Expr {
    type Output = Expr;
    fn try_mul(self, rhs: Expr) -> Option<Self::Output> {
        panic!();
    }
}
impl TryDiv<Expr> for Expr {
    type Output = Expr;
    fn try_div(self, rhs: Expr) -> Option<Self::Output> {
        panic!();
    }
}
impl TryPow<Expr> for Expr {
    type Output = Expr;
    fn try_pow(self, rhs: Expr) -> Option<Self::Output> {
        panic!();
    }
}
