use std::convert::From;
use std::convert::TryFrom;
use std::fmt;
use std::ops;

use super::Float;

#[derive(Debug, Clone, Copy)]
pub struct Real {
    num: i64,
    denum: i64,
}
impl Real {
    pub fn new(numerator: i64, denumerator: i64) -> Self {
        Real {
            num: numerator,
            denum: denumerator,
        }
        .simplify()
    }
    fn simplify(mut self) -> Self {
        let common_factor = gcd(self.num, self.denum);
        self.num /= common_factor;
        self.denum /= common_factor;
        self
    }
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.denum {
            1 => write!(f, "{}", self.num),
            _ => write!(f, "({}/{})", self.num, self.denum),
        }
    }
}

// Eq
impl Eq for Real {}

impl PartialEq for Real {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.denum == other.denum
    }
}

impl PartialEq<i64> for Real {
    fn eq(&self, other: &i64) -> bool {
        self.denum == 1 && self.num == *other
    }
}

// From
impl From<Real> for f64 {
    fn from(real: Real) -> f64 {
        real.num as f64 / real.denum as f64
    }
}

impl From<i64> for Real {
    fn from(value: i64) -> Real {
        Real {
            num: value,
            denum: 1,
        }
    }
}

impl TryFrom<Real> for i64 {
    type Error = &'static str;

    fn try_from(value: Real) -> Result<Self, Self::Error> {
        if value.denum != 1 {
            Err("Fractional Real connot be converted to i64")
        } else {
            Ok(value.num)
        }
    }
}

// Operators
impl ops::Add<Float> for Real {
    type Output = Float;

    fn add(self, rhs: Float) -> Self::Output {
        Float::from(self) + rhs
    }
}
impl ops::Add<Real> for Real {
    type Output = Real;

    fn add(self, rhs: Real) -> Self::Output {
        let new_den = (self.denum * rhs.denum) / gcd(self.denum, rhs.denum);
        let new_num = self.num * (new_den / self.denum) + rhs.num * (new_den / rhs.denum);
        Real::new(new_num, new_den).simplify()
    }
}

impl ops::Sub<Real> for Real {
    type Output = Real;

    fn sub(self, rhs: Real) -> Self::Output {
        let new_den = (self.denum * rhs.denum) / gcd(self.denum, rhs.denum);
        let new_num = self.num * (new_den / self.denum) - rhs.num * (new_den / rhs.denum);
        Real::new(new_num, new_den).simplify()
    }
}
impl ops::Sub<Float> for Real {
    type Output = Float;

    fn sub(self, rhs: Float) -> Self::Output {
        Float::from(self) - rhs
    }
}

impl ops::Mul<Float> for Real {
    type Output = Float;

    fn mul(self, rhs: Float) -> Self::Output {
        Float::from(self) * rhs
    }
}
impl ops::Mul<Real> for Real {
    type Output = Real;

    fn mul(self, rhs: Real) -> Self::Output {
        Real::new(self.num * rhs.num, self.denum * rhs.denum).simplify()
    }
}

impl ops::Div<Float> for Real {
    type Output = Float;

    fn div(self, rhs: Float) -> Self::Output {
        Float::from(self) / rhs
    }
}
impl ops::Div<Real> for Real {
    type Output = Real;

    fn div(self, rhs: Real) -> Self::Output {
        let inv_rhs = Real::new(rhs.denum, rhs.num);
        self * inv_rhs
    }
}

// Helpers
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a != 0 {
        let tmp = a;
        a = b % a;
        b = tmp;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(10, 13), 1);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(40, 100), 20);
        assert_eq!(gcd(4043634, 13242340), 2);
    }
    #[test]
    fn test_eq() {
        assert_eq!(Real::new(6, 4), Real::new(3, 2));
        assert_eq!(Real::new(-6, 4), Real::new(-3, 2));
        assert_eq!(Real::new(9, -99999), Real::new(1, -11111));
        assert_eq!(Real::new(9, -99999), Real::new(-1, 11111));
        assert_eq!(Real::new(-9, 99999), Real::new(-1, 11111));
        assert_eq!(Real::new(-1, -1), Real::new(1, 1));
        assert_eq!(Real::new(-9999, -3), Real::new(3333, 1));
        assert_ne!(Real::new(1, 2), Real::new(1, 3));
    }
}
