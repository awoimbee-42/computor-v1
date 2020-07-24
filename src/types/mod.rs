pub mod expr;
pub mod factor;
mod operators;
pub mod term;
pub mod value;

pub use expr::Expr;
pub use factor::Factor;
pub use term::Term;
pub use value::Num;
pub use value::Value;
pub use value::Var;

pub trait Resolve {
    fn resolve(&mut self) -> Option<Value>;
}

// #[derive(Debug, Clone)]
// pub struct Fun {
//     name: String,
//     vars: Vec<Value>,
//     inner: Group,
// }

#[macro_export]
macro_rules! uniq_resolve {
    ($self:ident, $value:ident) => {{
        let mut opt_v = $value.resolve();
        if let Some(new_v) = &opt_v {
            *$self = Self::from(new_v.clone());
        }
        opt_v
    }};
}
