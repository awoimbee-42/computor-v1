use std::ops;
use std::fmt;

#[derive(Debug)]
pub struct Real {
    num: u64,
    denum: u64,
}
impl Real {
    pub fn new(numerator: u64, denumerator: u64) -> Self {
        Real {
            num: numerator,
            denum: denumerator,
        }
    }
    /// Create a Real number from a Relative one
    pub fn from_rel(number: u64) -> Self {
        Real {
            num: number,
            denum: 1,
        }
    }
}

impl ops::Add<f64> for Real {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        let self_flt = self.num as f64 / self.denum as f64;
        self_flt + rhs
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
