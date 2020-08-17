use super::Num;
use super::Value;
use super::*;
use std::cmp;
use std::fmt;
use std::ops;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Var {
    name: String,
    coef: Num,
    pow: Num,
}

impl Var {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Var {
            name: name.into().to_lowercase(),
            coef: Num::from(1),
            pow: Num::from(1),
        }
    }
    fn eq_name(&self, other: &Self) -> bool {
        self.name == other.name
    }
    fn eq_pow(&self, other: &Self) -> bool {
        self.eq_name(other) && self.pow == other.pow
    }
    fn simplify(self) -> Value {
        if self.coef == 0 {
            Value::from(Num::from(0))
        } else if self.pow == 0 {
            Value::from(Num::from(1))
        } else {
            Value::from(self)
        }
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.coef != 1 {
            write!(f, "{}", self.coef)?;
        }
        write!(f, "{}", self.name)?;
        if self.pow != 1 {
            write!(f, "^{}", self.pow)?;
        }
        Ok(())
    }
}

impl cmp::PartialEq<Num> for Var {
    fn eq(&self, _rhs: &Num) -> bool {
        false
    }
}
impl cmp::PartialEq<Var> for Num {
    fn eq(&self, _rhs: &Var) -> bool {
        false
    }
}

impl TryAdd<Var> for Num {
    type Output = Value;
    fn try_add(&mut self, _rhs: &mut Var) -> Option<Self::Output> {
        None
    }
}
impl TrySub<Var> for Num {
    type Output = Value;
    fn try_sub(&mut self, _rhs: &mut Var) -> Option<Self::Output> {
        None
    }
}

impl TryAdd<Num> for Var {
    type Output = Value;

    fn try_add(&mut self, _rhs: &mut Num) -> Option<Self::Output> {
        None
    }
}
impl TrySub<Num> for Var {
    type Output = Value;

    fn try_sub(&mut self, _rhs: &mut Num) -> Option<Self::Output> {
        None
    }
}
impl TryAdd<Self> for Var {
    type Output = Var;
    fn try_add(&mut self, rhs: &mut Self) -> Option<Self::Output> {
        if self.eq_pow(&rhs) {
            self.coef = self.coef.clone() + rhs.coef.clone();
            Some(self.clone())
        } else {
            None
        }
    }
}
impl TrySub<Self> for Var {
    type Output = Var;
    fn try_sub(&mut self, rhs: &mut Self) -> Option<Self::Output> {
        if self.eq_pow(&rhs) {
            self.coef = self.coef.clone() - rhs.coef.clone();
            Some(self.clone())
        } else {
            None
        }
    }
}
impl super::TryPow<Var> for Num {
    type Output = Self;
    fn try_pow(&mut self, _rhs: &mut Var) -> Option<Self::Output> {
        None
    }
}
impl super::TryPow<Self> for Var {
    type Output = Self;
    fn try_pow(&mut self, _rhs: &mut Var) -> Option<Self::Output> {
        None
    }
}
impl TryDiv<Var> for Num {
    type Output = Self;
    fn try_div(&mut self, _rhs: &mut Var) -> Option<Self::Output> {
        None
    }
}
impl TryMul<Num> for Var {
    type Output = Self;
    fn try_mul(&mut self, rhs: &mut Num) -> Option<Self::Output> {
        self.coef = self.coef.clone() * rhs.clone();
        Some(self.clone())
    }
}
impl TryMul<Var> for Num {
    type Output = Var;
    fn try_mul(&mut self, rhs: &mut Var) -> Option<Self::Output> {
        rhs.try_mul(self)
    }
}
impl TryMul<Var> for Var {
    type Output = Self;
    fn try_mul(&mut self, rhs: &mut Self) -> Option<Self::Output> {
        if !self.eq_name(&rhs) {
            None
        } else {
            self.pow = self.pow.clone() * rhs.pow.clone();
            self.coef = self.coef.clone() + rhs.coef.clone();
            Some(self.clone())
        }
    }
}
impl TryDiv<Num> for Var {
    type Output = Self;
    fn try_div(&mut self, rhs: &mut Num) -> Option<Self::Output> {
        self.coef = self.coef.clone() / rhs.clone();
        Some(self.clone())
    }
}
impl TryDiv<Var> for Var {
    type Output = Self;
    fn try_div(&mut self, rhs: &mut Self) -> Option<Self::Output> {
        if !self.eq_name(&rhs) {
            None
        } else {
            self.pow = self.pow.clone() - rhs.pow.clone();
            self.coef = self.coef.clone() / rhs.coef.clone();
            Some(self.clone())
        }
    }
}
impl TryPow<Num> for Var {
    type Output = Self;
    fn try_pow(&mut self, rhs: &mut Num) -> Option<Self::Output> {
        self.pow = self.pow.clone() * rhs.clone();
        Some(self.clone())
    }
}
