use super::Num;
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

impl ops::Add<Num> for Var {
    type Output = Num;
    fn add(self, rhs: Num) -> Self::Output {
        panic!("I have to work around this... :("); // TODO
    }
}
impl ops::Add<Var> for Num {
    type Output = Num;
    fn add(self, rhs: Var) -> Self::Output {
        panic!("I have to work around this... :("); // TODO
    }
}
impl ops::Add<Var> for Var {
    type Output = Num;
    fn add(self, rhs: Var) -> Self::Output {
        panic!("I have to work around this... :("); // TODO
    }
}
impl ops::Sub<Num> for Var {
    type Output = Num;
    fn sub(self, rhs: Num) -> Self::Output {
        panic!("I have to work around this... :("); // TODO
    }
}
impl ops::Sub<Var> for Num {
    type Output = Num;
    fn sub(self, rhs: Var) -> Self::Output {
        panic!("I have to work around this... :("); // TODO
    }
}
impl ops::Sub<Var> for Var {
    type Output = Num;
    fn sub(self, rhs: Var) -> Self::Output {
        panic!("I have to work around this... :("); // TODO
    }
}
impl super::Pow<Var> for Num {
    type Output = Self;
    fn pow(self, rhs: Var) -> Self::Output {
        panic!("I have to work around this... :("); // TODO
    }
}
impl super::Pow<Var> for Var {
    type Output = Self;
    fn pow(self, rhs: Var) -> Self::Output {
        panic!("I have to work around this... :("); // TODO
    }
}
impl ops::Div<Var> for Num {
    type Output = Var;
    fn div(self, rhs: Var) -> Self::Output {
        panic!("I have to work around this... :("); // TODO
    }
}

impl ops::Mul<Num> for Var {
    type Output = Self;

    fn mul(mut self, rhs: Num) -> Self::Output {
        self.coef = self.coef * rhs;
        self
    }
}
impl ops::Mul<Var> for Num {
    type Output = Var;
    fn mul(self, rhs: Var) -> Self::Output {
        rhs * self
    }
}
impl ops::Mul<Var> for Var {
    type Output = Self;
    fn mul(mut self, rhs: Var) -> Self::Output {
        if !self.eq_name(&rhs) {
            panic!("TODO");
        }
        self.pow = self.pow * rhs.pow;
        self.coef = self.coef + rhs.coef;
        self
    }
}
impl ops::Div<Num> for Var {
    type Output = Self;
    fn div(mut self, rhs: Num) -> Self::Output {
        self.coef = self.coef / rhs;
        self
    }
}
impl ops::Div<Var> for Var {
    type Output = Self;
    fn div(mut self, rhs: Var) -> Self::Output {
        if !self.eq_name(&rhs) {
            panic!("TODO");
        }
        self.pow = self.pow - rhs.pow;
        self.coef = self.coef / rhs.coef;
        self
    }
}
impl super::Pow<Num> for Var {
    type Output = Self;
    fn pow(mut self, rhs: Num) -> Self::Output {
        self.pow = self.pow * rhs;
        self
    }
}
