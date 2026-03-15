use std::array;

use tracy_macros::{Neg, Random, fields, ops};
use tracy_traits::{Num, Random};

use crate::{
    arr_store::{self, ArrStore},
    num::lerp,
    vec::VecND,
};

/// N-dimensional point.
#[derive(Clone, Copy, Debug, Default, Random, Neg)]
#[fields(x, y, z)]
#[ops(
    add(lhs = VecND<T, N>, rhs = VecND<T, N>),
    sub(lhs = VecND<T, N>, rhs = VecND<T, N>),
    sub(out = VecND<T, N>)
)]
pub struct PointND<T: Num, const N: usize> {
    pub arr: ArrStore<T, N>,
}

impl<const N: usize> PointND<f64, N> {
    pub fn lerp(&self, rhs: &Self, t: f64) -> Self {
        Self {
            arr: ArrStore(array::from_fn(|i| lerp(self.arr[i], rhs.arr[i], t))),
        }
    }
}

/// 2D point (x, y).
pub type Point2D<T> = PointND<T, 2>;
/// 3D point (x, y, z).
pub type Point3D<T> = PointND<T, 3>;
