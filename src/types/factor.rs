use std::fmt;
use super::value::Value;
use super::value::Pow;
use super::value::Num;

use log::debug;

#[derive(Debug, Clone)]
pub enum Factor {
    Pow((Value, Box<Factor>)),
    Value(Value),
}

impl fmt::Display for Factor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Factor::Pow((v0, v1)) => write!(f, "{}^{}", v0, v1),
            Factor::Value(v) => write!(f, "{}", v),
        }
    }
}

impl<T: Into<Value>> From<T> for Factor {
	fn from(val: T) -> Self {
		Factor::Value(val.into())
	}
}

impl super::Resolve for Factor {
	type Output = Num;

	fn resolve(&mut self) -> Option<Self::Output> {
		debug!("resolve: {}", self);
		fn inner(s: &mut Factor) -> Option<Num> {
			match s {
				Factor::Value(v) => return v.resolve(),
				Factor::Pow((v, f)) => {
					let f = f.resolve();
					let v = v.resolve();
					if let Some(f) = f {
						if f == 0 {
							return Some(Num::from(0));
						} else if let Some(n) = v {
							return Some(n.clone().pow(f));
						}
					}
				}
			}
			return None;
		}
		let res = inner(self);
		if let Some(n) = &res {
			*self = Self::Value(Value::from(n.clone()));
		}
		res
	}
}
