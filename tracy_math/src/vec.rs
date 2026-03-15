use tracy_macros::{Neg, Random, fields, ops};
use tracy_traits::{Num, Random};

use crate::arr_store::{self, ArrStore};

/// N-dimensional vector.
#[derive(Clone, Copy, Debug, Default, Random, Neg)]
#[fields(x, y, z, w)]
#[ops(
    add,
    sub,
    mul(lhs = T),
    div(rhs = T)
)]
pub struct VecND<T: Num, const N: usize> {
    pub arr: ArrStore<T, N>,
}

impl<T: Num, const N: usize> VecND<T, N> {
    pub fn len_2(&self) -> T {
        self.arr.iter().map(|&val| val * val).sum()
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.arr
            .iter()
            .zip(rhs.arr.iter())
            .map(|(&lhs, &rhs)| lhs * rhs)
            .sum()
    }
}

impl<const N: usize> VecND<f64, N> {
    pub fn norm(&self) -> Self {
        *self / self.len_2().sqrt()
    }
}

impl<T: Num> VecND<T, 3> {
    pub fn cross(&self, rhs: &Self) -> Self {
        let x = self.y() * rhs.z() - self.z() * rhs.y();
        let y = self.z() * rhs.x() - self.x() * rhs.z();
        let z = self.x() * rhs.y() - self.y() * rhs.x();
        Self::new(x, y, z)
    }
}

/// 2D vector (x, y).
pub type Vec2D<T> = VecND<T, 2>;
/// 3D vector (x, y ,z).
pub type Vec3D<T> = VecND<T, 3>;
