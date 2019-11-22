use crate::{
    util::{marker, Conversion, Dimension, Quantity, Units},
    vector3,
    vectors::*,
};

use core::ops::{Div, Sub};

use alga::general::ClosedDiv;
use nalgebra::base::Scalar;
use num_traits::Num;

impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Dl::Kind: marker::Div,
    Dr::Kind: marker::Div,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedDiv,
    Dl::L: Sub<Dr::L>,
    Dl::M: Sub<Dr::M>,
    Dl::T: Sub<Dr::T>,
    Dl::I: Sub<Dr::I>,
    Dl::Th: Sub<Dr::Th>,
    Dl::N: Sub<Dr::N>,
    Dl::J: Sub<Dr::J>,
{
    type Output = DimVector3<DiffDimension<Dl, Dr>, Ul, V>;
    fn div(self, rhs: Quantity<Dr, Ur, V>) -> Self::Output {
        vector3!(self.value / rhs.value)
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<&'r Quantity<Dr, Ur, V>>
    for &'l DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedDiv,
    Dl::Kind: marker::Div,
    Dr::Kind: marker::Div,
    Dl::L: Sub<Dr::L> + 'r,
    Dl::M: Sub<Dr::M> + 'r,
    Dl::T: Sub<Dr::T> + 'r,
    Dl::I: Sub<Dr::I> + 'r,
    Dl::Th: Sub<Dr::Th> + 'r,
    Dl::N: Sub<Dr::N> + 'r,
    Dl::J: Sub<Dr::J> + 'r,
{
    type Output = DimVector3<DiffDimension<Dl, Dr>, Ul, V>;
    fn div(self, rhs: &Quantity<Dr, Ur, V>) -> Self::Output {
        vector3!(self.value / rhs.value)
    }
}
impl<'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<&'r Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedDiv,
    Dl::Kind: marker::Div,
    Dr::Kind: marker::Div,
    Dl::L: Sub<Dr::L> + 'r,
    Dl::M: Sub<Dr::M> + 'r,
    Dl::T: Sub<Dr::T> + 'r,
    Dl::I: Sub<Dr::I> + 'r,
    Dl::Th: Sub<Dr::Th> + 'r,
    Dl::N: Sub<Dr::N> + 'r,
    Dl::J: Sub<Dr::J> + 'r,
{
    type Output = DimVector3<DiffDimension<Dl, Dr>, Ul, V>;
    fn div(self, rhs: &Quantity<Dr, Ur, V>) -> Self::Output {
        vector3!(self.value / rhs.value)
    }
}
impl<D: ?Sized, U: ?Sized, V> Div<V> for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedDiv,
    D::Kind: marker::Div,
{
    type Output = Self;
    fn div(self, rhs: V) -> Self::Output {
        vector3!(self.value / rhs)
    }
}
