use super::Term;
use super::Num;

use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Expr {
	name: Option<String>,
	vars: Option<HashMap<String, Num>>,
	expr: ExprInner,
}

#[derive(Debug, Clone)]
enum ExprInner {
    Add((Box<Expr>, Term)),
    Sub((Box<Expr>, Term)),
    Term(Term),
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(vars) = self.vars {
			for (k, v) in vars {
				writeln!(f, "{}: {}", k, v);
			}
		}
        match self.expr {
            ExprInner::Add((v0, v1)) => write!(f, "{} + {}", v0, v1),
            ExprInner::Sub((v0, v1)) => write!(f, "{} - {}", v0, v1),
            ExprInner::Term(v) => write!(f, "{}", v),
        }
    }
}

impl From<Term> for Expr {
	fn from(v: Term) -> Self {
		Expr {
			name: None,
			vars: None,
			expr: ExprInner::Term(v)
		}
	}
}

impl Expr {
	pub fn new_add<E>(e: E, t: Term) -> Self where E: Into<Box<Expr>> {
		Expr {
			name: None,
			vars: None,
			expr: ExprInner::Add((e.into(), t))
		}
	}
	pub fn new_sub<E>(e: E, t: Term) -> Self where E: Into<Box<Expr>> {
		Expr {
			name: None,
			vars: None,
			expr: ExprInner::Sub((e.into(), t))
		}
	}
}
