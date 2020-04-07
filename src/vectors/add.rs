use crate::{
    util::{marker, Conversion, Dimension, Units},
    vectors::DimVector3,
};

use core::ops::Add;

use nalgebra::{base::Scalar, ClosedAdd};
use num_traits::Num;

impl<D: ?Sized, U: ?Sized, V> Add<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Add,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedAdd,
{
    type Output = DimVector3<D, U, V>;
    fn add(self, rhs: DimVector3<D, U, V>) -> Self::Output {
        Add::add(self.value, rhs.value).into()
    }
}
impl<'l, D: ?Sized, U: ?Sized, V> Add<DimVector3<D, U, V>> for &'l DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Add,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedAdd,
{
    type Output = DimVector3<D, U, V>;
    fn add(self, rhs: DimVector3<D, U, V>) -> Self::Output {
        Add::add(self.value, rhs.value).into()
    }
}
impl<'l, D: ?Sized, U: ?Sized, V> Add<DimVector3<D, U, V>> for &'l mut DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Add,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedAdd,
{
    type Output = DimVector3<D, U, V>;
    fn add(self, rhs: DimVector3<D, U, V>) -> Self::Output {
        Add::add(self.value, rhs.value).into()
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> Add<&'r DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Add,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedAdd,
    // type Norm = Quantity<D, U, <V as ComplexField>::RealField>;
{
    type Output = DimVector3<D, U, V>;
    fn add(self, rhs: &DimVector3<D, U, V>) -> Self::Output {
        Add::add(self.value, rhs.value).into()
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> Add<&'r DimVector3<D, U, V>> for &'l DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Add,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedAdd,
{
    type Output = DimVector3<D, U, V>;
    fn add(self, rhs: &DimVector3<D, U, V>) -> Self::Output {
        Add::add(self.value, rhs.value).into()
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> Add<&'r DimVector3<D, U, V>> for &'l mut DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Add,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedAdd,
{
    type Output = DimVector3<D, U, V>;
    fn add(self, rhs: &DimVector3<D, U, V>) -> Self::Output {
        Add::add(self.value, rhs.value).into()
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> Add<&'r mut DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Add,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedAdd,
{
    type Output = DimVector3<D, U, V>;
    fn add(self, rhs: &mut DimVector3<D, U, V>) -> Self::Output {
        Add::add(self.value, rhs.value).into()
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> Add<&'r mut DimVector3<D, U, V>> for &'l DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Add,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedAdd,
{
    type Output = DimVector3<D, U, V>;
    fn add(self, rhs: &mut DimVector3<D, U, V>) -> Self::Output {
        Add::add(self.value, rhs.value).into()
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> Add<&'r mut DimVector3<D, U, V>>
    for &'l mut DimVector3<D, U, V>
where
    D: Dimension,
    D::Kind: marker::Add,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedAdd,
{
    type Output = DimVector3<D, U, V>;
    fn add(self, rhs: &mut DimVector3<D, U, V>) -> Self::Output {
        Add::add(self.value, rhs.value).into()
    }
}
