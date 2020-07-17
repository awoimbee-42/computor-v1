pub mod expr;
pub mod factor;
pub mod term;
pub mod value;
mod operators;

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
