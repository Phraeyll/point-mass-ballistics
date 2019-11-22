use crate::{
    util::{marker, Conversion, Dimension, Units},
    vectors::DimVector3,
};

use core::ops::SubAssign;

use alga::general::ClosedSub;
use nalgebra::base::Scalar;
use num_traits::Num;

impl<D: ?Sized, U: ?Sized, V> SubAssign<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedSub,
    D::Kind: marker::SubAssign,
{
    fn sub_assign(&mut self, rhs: DimVector3<D, U, V>) {
        self.value -= rhs.value
    }
}
impl<'l, D: ?Sized, U: ?Sized, V> SubAssign<DimVector3<D, U, V>> for &'l mut DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedSub,
    D::Kind: marker::SubAssign,
{
    fn sub_assign(&mut self, rhs: DimVector3<D, U, V>) {
        self.value -= rhs.value
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> SubAssign<&'r DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedSub,
    D::Kind: marker::SubAssign,
{
    fn sub_assign(&mut self, rhs: &DimVector3<D, U, V>) {
        self.value -= rhs.value
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> SubAssign<&'r DimVector3<D, U, V>>
    for &'l mut DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedSub,
    D::Kind: marker::SubAssign,
{
    fn sub_assign(&mut self, rhs: &DimVector3<D, U, V>) {
        self.value -= rhs.value
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> SubAssign<&'r mut DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedSub,
    D::Kind: marker::SubAssign,
{
    fn sub_assign(&mut self, rhs: &mut DimVector3<D, U, V>) {
        self.value -= rhs.value
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> SubAssign<&'r mut DimVector3<D, U, V>>
    for &'l mut DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedSub,
    D::Kind: marker::SubAssign,
{
    fn sub_assign(&mut self, rhs: &mut DimVector3<D, U, V>) {
        self.value -= rhs.value
    }
}
