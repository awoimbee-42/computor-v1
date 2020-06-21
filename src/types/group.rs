use super::Token;
use crate::operators::Operator;
use log::debug;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Group(Vec<Token>);
impl Group {
    pub fn new() -> Self {
        Group(Vec::new())
    }
    pub fn simplify_ref(&mut self) {
        debug!("simplify group: {}", &self);
        for precedence_lvl in (0..8).rev() {
            debug!("precedence {}", precedence_lvl);
            let mut has_mutated = true;
            while has_mutated {
                has_mutated = false;
                for (id, token) in self.iter().enumerate() {
                    if let Token::Operator(o) = token {
                        if o.precedence() != precedence_lvl {
                            continue;
                        }
                        debug!("Operator {:?}", o);
                        let mut group = self.clone();
                        if o.operate(&mut group, id).is_ok() {
                            debug!("Operation successful");
                            self.0 = group.0;
                            has_mutated = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn simplify(mut self) -> Self {
        self.simplify_ref();
        self
    }
}

impl std::ops::Deref for Group {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Group {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        for v in &self.0 {
            write!(f, "{}", v)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}
