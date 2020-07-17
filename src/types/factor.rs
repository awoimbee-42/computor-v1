use std::fmt;
use super::value::Value;

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

impl super::Resolve for Factor {
	type Output = Value;

	fn resolve(self) -> Result<Self::Output, Self>
	where Self: Sized {
		match self {
			Self::Value(v) => return Ok(v),
			Self::Pow((v, f)) => {
				if let Ok(f) = f.resolve() {
					return Ok(v.pow(f));
				} else {
					return Err(self);
				}
			}
		}
	}
}
