use super::Real;
use std::convert::From;
use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Float(f64);

impl Float {
    pub fn new(val: f64) -> Self {
        Float(val)
    }
}

impl std::ops::Deref for Float {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Eq
impl Eq for Float {}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

// From
impl From<i64> for Float {
    fn from(val: i64) -> Self {
        Self(val as f64)
    }
}
impl From<Real> for Float {
    fn from(val: Real) -> Self {
        Self(f64::from(val))
    }
}
impl From<f64> for Float {
    fn from(val: f64) -> Self {
        Self(val)
    }
}

// Operators
impl ops::Add<Real> for Float {
    type Output = Self;

    fn add(self, rhs: Real) -> Self::Output {
        Float::from(rhs) + self
    }
}
impl ops::Add<Float> for Float {
    type Output = Float;

    fn add(self, rhs: Float) -> Self::Output {
        Self(*self + *rhs)
    }
}

impl ops::Sub<Real> for Float {
    type Output = Self;

    fn sub(self, rhs: Real) -> Self::Output {
        Float::from(rhs) - self
    }
}
impl ops::Sub<Float> for Float {
    type Output = Float;

    fn sub(self, rhs: Float) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl ops::Mul<Float> for Float {
    type Output = Float;

    fn mul(self, rhs: Float) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl ops::Mul<Real> for Float {
    type Output = Float;

    fn mul(self, rhs: Real) -> Self::Output {
        Self(self.0 * Float::from(rhs).0)
    }
}

impl ops::Div<Float> for Float {
    type Output = Float;

    fn div(self, rhs: Float) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
impl ops::Div<Real> for Float {
    type Output = Float;

    fn div(self, rhs: Real) -> Self::Output {
        Self(self.0 / Float::from(rhs).0)
    }
}
use std::convert::TryFrom;
impl super::Pow<Real> for Float {
    type Output = Float;

    fn pow(mut self, rhs: Real) -> Self::Output {
        self.0 = self.0.powi(i64::try_from(rhs).unwrap() as i32);
        self
    }
}
impl super::Pow<Float> for Float {
    type Output = Float;

    fn pow(self, rhs: Float) -> Self::Output {
        Float::from(self.powf(rhs.0))
    }
}
