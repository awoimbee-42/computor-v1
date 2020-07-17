pub mod expr;
pub mod factor;
pub mod term;
pub mod value;

pub use expr::Expr;
pub use factor::Factor;
pub use term::Term;
pub use value::Value;
pub use value::Num;
pub use value::Var;

pub trait Resolve {
    type Output;

    fn resolve(&mut self) -> Option<Self::Output>;
}

// #[derive(Debug, Clone)]
// pub struct Fun {
//     name: String,
//     vars: Vec<Value>,
//     inner: Group,
// }
