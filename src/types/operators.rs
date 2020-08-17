pub trait TryAdd<T> {
    type Output;

    fn try_add(&mut self, rhs: &mut T) -> Option<Self::Output>;
}
pub trait TrySub<T> {
    type Output;

    fn try_sub(&mut self, rhs: &mut T) -> Option<Self::Output>;
}
pub trait TryMul<T> {
    type Output;

    fn try_mul(&mut self, rhs: &mut T) -> Option<Self::Output>;
}
pub trait TryDiv<T> {
    type Output;

    fn try_div(&mut self, rhs: &mut T) -> Option<Self::Output>;
}
pub trait TryPow<T> {
    type Output;

    fn try_pow(&mut self, rhs: &mut T) -> Option<Self::Output>;
}
