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
