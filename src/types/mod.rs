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
    fn resolve(&mut self) -> Option<Value>;
}

pub trait TryAdd<T> {
    type Output;

    fn try_add(self, rhs: T) -> Option<Self::Output>;
}
pub trait TrySub<T> {
    type Output;

    fn try_sub(self, rhs: T) -> Option<Self::Output>;
}
pub trait TryMul<T> {
    type Output;

    fn try_mul(self, rhs: T) -> Option<Self::Output>;
}
pub trait TryDiv<T> {
    type Output;

    fn try_div(self, rhs: T) -> Option<Self::Output>;
}
pub trait TryPow<T> {
    type Output;

    fn try_pow(self, rhs: T) -> Option<Self::Output>;
}


// #[derive(Debug, Clone)]
// pub struct Fun {
//     name: String,
//     vars: Vec<Value>,
//     inner: Group,
// }
