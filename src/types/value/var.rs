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
    fn try_add(self, _rhs: Var) -> Option<Self::Output> {
        None
    }
}
impl TrySub<Var> for Num {
    type Output = Value;
    fn try_sub(self, _rhs: Var) -> Option<Self::Output> {
        None
    }
}

impl TryAdd<Num> for Var {
    type Output = Value;

    fn try_add(self, _rhs: Num) -> Option<Self::Output> {
        None
    }
}
impl TrySub<Num> for Var {
    type Output = Value;

    fn try_sub(self, _rhs: Num) -> Option<Self::Output> {
        None
    }
}
impl TryAdd<Self> for Var {
    type Output = Var;
    fn try_add(mut self, rhs: Self) -> Option<Self::Output> {
        if self.eq_pow(&rhs) {
            self.coef = self.coef + rhs.coef;
            Some(self)
        } else {
            None
        }
    }
}
impl TrySub<Self> for Var {
    type Output = Var;
    fn try_sub(mut self, rhs: Self) -> Option<Self::Output> {
        if self.eq_pow(&rhs) {
            self.coef = self.coef - rhs.coef;
            Some(self)
        } else {
            None
        }
    }
}
impl super::TryPow<Var> for Num {
    type Output = Self;
    fn try_pow(self, _rhs: Var) -> Option<Self::Output> {
        None
    }
}
impl super::TryPow<Self> for Var {
    type Output = Self;
    fn try_pow(self, _rhs: Var) -> Option<Self::Output> {
        None
    }
}
impl TryDiv<Var> for Num {
    type Output = Self;
    fn try_div(self, _rhs: Var) -> Option<Self::Output> {
        None
    }
}
impl TryMul<Num> for Var {
    type Output = Self;
    fn try_mul(mut self, rhs: Num) -> Option<Self::Output> {
        self.coef = self.coef * rhs;
        Some(self)
    }
}
impl TryMul<Var> for Num {
    type Output = Var;
    fn try_mul(self, rhs: Var) -> Option<Self::Output> {
        rhs.try_mul(self)
    }
}
impl TryMul<Var> for Var {
    type Output = Self;
    fn try_mul(mut self, rhs: Self) -> Option<Self::Output> {
        if !self.eq_name(&rhs) {
            None
        } else {
            self.pow = self.pow * rhs.pow;
            self.coef = self.coef + rhs.coef;
            Some(self)
        }
    }
}
impl TryDiv<Num> for Var {
    type Output = Self;
    fn try_div(mut self, rhs: Num) -> Option<Self::Output> {
        self.coef = self.coef / rhs;
        Some(self)
    }
}
impl TryDiv<Var> for Var {
    type Output = Self;
    fn try_div(mut self, rhs: Self) -> Option<Self::Output> {
        if !self.eq_name(&rhs) {
            None
        } else {
            self.pow = self.pow - rhs.pow;
            self.coef = self.coef / rhs.coef;
            Some(self)
        }
    }
}
impl TryPow<Num> for Var {
    type Output = Self;
    fn try_pow(mut self, rhs: Num) -> Option<Self::Output> {
        self.pow = self.pow * rhs;
        Some(self)
    }
}
