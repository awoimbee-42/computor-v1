use std::fmt;
use super::factor::Factor;

#[derive(Debug, Clone)]
pub enum Term {
    Mul((Box<Term>, Factor)),
    Div((Box<Term>, Factor)),
    Factor(Factor),
}
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Mul((v0, v1)) => write!(f, "{} * {}", v0, v1),
            Term::Div((v0, v1)) => write!(f, "{} / {}", v0, v1),
            Term::Factor(v) => write!(f, "{}", v),
        }
    }
}
