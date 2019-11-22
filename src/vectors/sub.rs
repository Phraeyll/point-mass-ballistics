use crate::{
    util::{marker, Conversion, Dimension, Units},
    vector3,
    vectors::DimVector3,
};

use core::ops::Sub;

use alga::general::ClosedSub;
use nalgebra::base::Scalar;
use num_traits::Num;

impl<D: ?Sized, U: ?Sized, V> Sub for DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedSub,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        vector3!(self.value - rhs.value)
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> Sub<&'r Self> for &'l DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedSub,
{
    type Output = DimVector3<D, U, V>;
    fn sub(self, rhs: &Self) -> Self::Output {
        vector3!(self.value - rhs.value)
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> Sub<&'r Self> for DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedSub,
{
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        vector3!(self.value - rhs.value)
    }
}
impl<'l, D: ?Sized, U: ?Sized, V> Sub<DimVector3<D, U, V>> for &'l DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedSub,
{
    type Output = DimVector3<D, U, V>;
    fn sub(self, rhs: DimVector3<D, U, V>) -> Self::Output {
        vector3!(self.value - rhs.value)
    }
}
