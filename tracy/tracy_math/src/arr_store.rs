use std::{
    array,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use tracy_traits::Num;

#[derive(Clone, Copy, Debug)]
pub struct ArrStore<T: Num, const N: usize>(pub [T; N]);

impl<T: Num, const N: usize> Deref for ArrStore<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Num, const N: usize> DerefMut for ArrStore<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Num, const N: usize> Index<usize> for ArrStore<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Num, const N: usize> IndexMut<usize> for ArrStore<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Num, const N: usize> Default for ArrStore<T, N> {
    fn default() -> Self {
        Self([T::default(); N])
    }
}

impl<T: Num, const N: usize> IntoIterator for ArrStore<T, N> {
    type Item = T;
    type IntoIter = array::IntoIter<Self::Item, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
