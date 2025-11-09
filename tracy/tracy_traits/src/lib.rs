use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use rand::distr::uniform::SampleRange;

pub trait Random<T: Num> {
    fn random() -> Self;
    fn random_range<R: Clone + SampleRange<T>>(r: R) -> Self;
}

pub trait Num:
    Copy
    + Default
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Sum
{
}

impl Num for f64 {}
impl Num for u8 {}

pub trait Color<T: Num> {
    // TODO: Find better name to differenciate 0.0-1.0 and 0-255 scales.
    type UColor: Color<u8>;
    fn to_gamma(&self) -> Self;
    fn scale(&self) -> Self::UColor;
}
