use std::ops::Range;
pub use tracy_macros_attr::{add, div, mul, sub};
pub use tracy_macros_derive::{Neg, Random};

pub trait Random<T> {
    fn random() -> Self;
    fn random_range(r: Range<T>) -> Self;
}
