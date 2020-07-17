mod float;
mod real;

use float::Float;
use real::Real;

use super::*;
use std::fmt;
use std::ops;

#[derive(Debug, Clone)]
pub enum Num {
    Float(Float),
    Real(Real),
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Num::Real(v) => write!(f, "{}", v),
            Num::Float(v) => write!(f, "{}", v),
        }
    }
}

macro_rules! for_any_num {
    ($matched:ident, $name:ident, $what:expr) => {
        match $matched {
            Num::Float($name) => $what,
            Num::Real($name) => $what,
        }
    };
}

/*################################### FROM ####################################*/
impl From<Real> for Num {
    fn from(val: Real) -> Self {
        Num::Real(val)
    }
}
impl From<Float> for Num {
    fn from(val: Float) -> Self {
        Num::Float(val)
    }
}
impl From<i64> for Num {
    fn from(val: i64) -> Self {
        Num::Real(Real::from(val))
    }
}
impl From<f64> for Num {
    fn from(val: f64) -> Self {
        Num::Float(Float::from(val))
    }
}

/*#################################### EQ ####################################*/
impl Eq for Num {}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        for_any_num!(self, v0, for_any_num!(other, v1, v0 == v1))
    }
}
impl PartialEq<i64> for Num {
    fn eq(&self, other: &i64) -> bool {
        for_any_num!(self, v0, v0 == other)
    }
}

/*#################################### OP ####################################*/
impl ops::Add<Num> for Num {
    type Output = Num;

    fn add(self, rhs: Num) -> Self::Output {
        for_any_num!(self, v0, for_any_num!(rhs, v1, Num::from(v0 + v1)))
    }
}
impl ops::Sub<Num> for Num {
    type Output = Num;

    fn sub(self, rhs: Num) -> Self::Output {
        for_any_num!(self, v0, for_any_num!(rhs, v1, Num::from(v0 - v1)))
    }
}
impl ops::Mul<Num> for Num {
    type Output = Num;

    fn mul(self, rhs: Num) -> Self::Output {
        for_any_num!(self, v0, for_any_num!(rhs, v1, Num::from(v0 * v1)))
    }
}
impl ops::Div<Num> for Num {
    type Output = Num;

    fn div(self, rhs: Num) -> Self::Output {
        for_any_num!(self, v0, for_any_num!(rhs, v1, Num::from(v0 / v1)))
    }
}
impl super::Pow<Num> for Num {
    type Output = Num;

    fn pow(self, rhs: Num) -> Self::Output {
        for_any_num!(self, v0, for_any_num!(rhs, v1, Num::from(v0.pow(v1))))
    }
}

/*################################## TRY OP ##################################*/
impl TryAdd<Num> for Num {
    type Output = Num;
    fn try_add(self, rhs: Num) -> Option<Self::Output> {
        for_any_num!(self, v0, for_any_num!(rhs, v1, Some(Num::from(v0 + v1))))
    }
}
impl TrySub<Num> for Num {
    type Output = Num;
    fn try_sub(self, rhs: Num) -> Option<Self::Output> {
        for_any_num!(self, v0, for_any_num!(rhs, v1, Some(Num::from(v0 - v1))))
    }
}
impl TryMul<Num> for Num {
    type Output = Num;
    fn try_mul(self, rhs: Num) -> Option<Self::Output> {
        for_any_num!(self, v0, for_any_num!(rhs, v1, Some(Num::from(v0 * v1))))
    }
}
impl TryDiv<Num> for Num {
    type Output = Num;
    fn try_div(self, rhs: Num) -> Option<Self::Output> {
        for_any_num!(self, v0, for_any_num!(rhs, v1, Some(Num::from(v0 / v1))))
    }
}
impl TryPow<Num> for Num {
    type Output = Num;
    fn try_pow(self, rhs: Num) -> Option<Self::Output> {
        for_any_num!(self, v0, for_any_num!(rhs, v1, Some(Num::from(v0.pow(v1)))))
    }
}
