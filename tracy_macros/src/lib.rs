use std::ops::Range;
pub use tracy_macros_impl::{Neg, Random, add, div, mul, sub};

pub trait Random<T> {
    fn random() -> Self;
    fn random_range(r: Range<T>) -> Self;
}
