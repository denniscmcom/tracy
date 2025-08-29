use std::ops::Range;
pub use tracy_macros_attr::{add, div, mul, ops, sub};
pub use tracy_macros_derive::{Color, Convert, Neg, Random};

pub trait Random<T> {
    fn random() -> Self;
    fn random_range(r: Range<T>) -> Self;
}

pub trait Convert<T, const N: usize> {
    fn as_array(&self) -> [T; N];
}

pub trait Color {
    type U8;
    fn to_u8(self) -> Self::U8;
    fn to_gamma(&self) -> Self;
}
