use crate::{
    util::{marker, Conversion, Dimension, Units},
    vectors::DimVector3,
};

use core::ops::AddAssign;

use alga::general::ClosedAdd;
use nalgebra::base::Scalar;
use num_traits::Num;

impl<D: ?Sized, U: ?Sized, V> AddAssign for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedAdd,
    D::Kind: marker::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> AddAssign<&'r Self> for &'l mut DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedAdd,
    D::Kind: marker::AddAssign,
{
    fn add_assign(&mut self, rhs: &Self) {
        self.value += rhs.value
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> AddAssign<&'r Self> for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedAdd,
    D::Kind: marker::AddAssign,
{
    fn add_assign(&mut self, rhs: &Self) {
        self.value += rhs.value
    }
}
impl<'l, D: ?Sized, U: ?Sized, V> AddAssign<DimVector3<D, U, V>> for &'l mut DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedAdd,
    D::Kind: marker::AddAssign,
{
    fn add_assign(&mut self, rhs: DimVector3<D, U, V>) {
        self.value += rhs.value
    }
}
