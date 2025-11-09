use tracy_macros::{Neg, Random, fields, ops};
use tracy_traits::{Num, Random};

use crate::arr_store::{self, ArrStore};

// TODO: Implement a wrapper for normalized quantities.

/// A wrapper for dimensionless quantities, typically produced by operations on
/// dimensional quantities.
#[derive(Clone, Copy, Debug, Default, Random, Neg)]
#[fields(raw)]
#[ops(add, sub, mul, div)]
pub struct Unitless<T: Num> {
    arr: ArrStore<T, 1>,
}

/// Degrees of arc.
#[derive(Clone, Copy, Debug, Default, Random, Neg)]
#[ops(
    add,
    sub,
    mul,
    mul(lhs = T),
    div(out = Unitless<T>),
    div(rhs = T)
)]
#[fields(raw)]
pub struct Degrees<T: Num> {
    arr: ArrStore<T, 1>,
}

impl Degrees<f64> {
    pub fn to_radians(self) -> Radians<f64> {
        Radians {
            arr: ArrStore([f64::to_radians(self.raw())]),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Random, Neg)]
#[ops(
    add,
    sub,
    mul,
    mul(lhs = T),
    div(out = Unitless<T>),
    div(rhs = T)
)]
#[fields(raw)]
pub struct Radians<T: Num> {
    arr: ArrStore<T, 1>,
}

impl Radians<f64> {
    pub fn to_degrees(self) -> Degrees<f64> {
        Degrees {
            arr: ArrStore([f64::to_degrees(self.raw())]),
        }
    }

    pub fn tan(self) -> f64 {
        f64::tan(self.raw())
    }
}
