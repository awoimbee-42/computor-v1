use super::operators::*;
use super::value::Value;
use crate::uniq_resolve;
use std::fmt;

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

impl From<Value> for Factor {
    fn from(val: Value) -> Self {
        Factor::Value(val.into())
    }
}

impl super::Resolve for Factor {
    fn resolve(&mut self) -> Option<Value> {
        debug!("resolve: {}", self);
        match self {
            Self::Value(v) => return uniq_resolve!(self, v),
            Self::Pow((a, b)) => {
                let opt_a = a.resolve();
                let opt_b = b.resolve();

                if let Some(num_b) = &opt_b {
                    if *num_b == 0 {
                        let res = Value::from(1);
                        *self = Self::from(res.clone());
                        return Some(res);
                    }
                    if *num_b == 1 {
                        *self = Factor::from(a.clone());
                        return self.resolve();
                    }
                }
                if let Some(num_a) = &opt_a {
                    if *num_a == 0 {
                        let res = Value::from(0);
                        *self = Self::from(res.clone());
                        return Some(res);
                    }
                    if *num_a == 1 {
                        let res = Value::from(1);
                        *self = Self::from(res.clone());
                        return Some(res);
                    }
                }
                if let Some(mut num_b) = opt_b {
                    if let Some(mut num_a) = opt_a {
                        let res = num_a.try_pow(&mut num_b);
                        if let Some(v) = &res {
                            *self = Self::from(v.clone());
                        }
                        return res;
                    }
                }
            }
        }
        return None;
    }
}
