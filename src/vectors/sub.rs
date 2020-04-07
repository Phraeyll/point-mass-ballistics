use crate::{
    util::{marker, Conversion, Dimension, Units},
    vectors::DimVector3,
};

use core::ops::Sub;

use nalgebra::{base::Scalar, ClosedSub};
use num_traits::Num;

impl<D: ?Sized, U: ?Sized, V> Sub<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedSub,
{
    type Output = DimVector3<D, U, V>;
    fn sub(self, rhs: DimVector3<D, U, V>) -> Self::Output {
        Sub::sub(self.value, rhs.value).into()
    }
}
impl<'l, D: ?Sized, U: ?Sized, V> Sub<DimVector3<D, U, V>> for &'l DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedSub,
{
    type Output = DimVector3<D, U, V>;
    fn sub(self, rhs: DimVector3<D, U, V>) -> Self::Output {
        Sub::sub(self.value, rhs.value).into()
    }
}
impl<'l, D: ?Sized, U: ?Sized, V> Sub<DimVector3<D, U, V>> for &'l mut DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedSub,
{
    type Output = DimVector3<D, U, V>;
    fn sub(self, rhs: DimVector3<D, U, V>) -> Self::Output {
        Sub::sub(self.value, rhs.value).into()
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> Sub<&'r DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedSub,
{
    type Output = DimVector3<D, U, V>;
    fn sub(self, rhs: &DimVector3<D, U, V>) -> Self::Output {
        Sub::sub(self.value, rhs.value).into()
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> Sub<&'r DimVector3<D, U, V>> for &'l DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedSub,
{
    type Output = DimVector3<D, U, V>;
    fn sub(self, rhs: &DimVector3<D, U, V>) -> Self::Output {
        Sub::sub(self.value, rhs.value).into()
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> Sub<&'r DimVector3<D, U, V>> for &'l mut DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedSub,
{
    type Output = DimVector3<D, U, V>;
    fn sub(self, rhs: &DimVector3<D, U, V>) -> Self::Output {
        Sub::sub(self.value, rhs.value).into()
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> Sub<&'r mut DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedSub,
{
    type Output = DimVector3<D, U, V>;
    fn sub(self, rhs: &mut DimVector3<D, U, V>) -> Self::Output {
        Sub::sub(self.value, rhs.value).into()
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> Sub<&'r mut DimVector3<D, U, V>> for &'l DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedSub,
{
    type Output = DimVector3<D, U, V>;
    fn sub(self, rhs: &mut DimVector3<D, U, V>) -> Self::Output {
        Sub::sub(self.value, rhs.value).into()
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> Sub<&'r mut DimVector3<D, U, V>>
    for &'l mut DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Sub,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedSub,
{
    type Output = DimVector3<D, U, V>;
    fn sub(self, rhs: &mut DimVector3<D, U, V>) -> Self::Output {
        Sub::sub(self.value, rhs.value).into()
    }
}
