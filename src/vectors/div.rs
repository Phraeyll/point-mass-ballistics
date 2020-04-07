use crate::{
    util::{marker, Conversion, Dimension, Quantity, Units},
    vectors::*,
};

use core::ops::{Div, Sub};

use nalgebra::{base::Scalar, ClosedDiv};
use num_traits::Num;

impl<D: ?Sized, U: ?Sized, V> Div<V> for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
    D::Kind: marker::Div,
{
    type Output = DimVector3<D, U, V>;
    fn div(self, rhs: V) -> Self::Output {
        Div::div(self.value, rhs).into()
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> Div<&'r V> for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
    D::Kind: marker::Div,
{
    type Output = DimVector3<D, U, V>;
    fn div(self, rhs: &V) -> Self::Output {
        Div::div(self.value, *rhs).into()
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> Div<&'r mut V> for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
    D::Kind: marker::Div,
{
    type Output = DimVector3<D, U, V>;
    fn div(self, rhs: &mut V) -> Self::Output {
        Div::div(self.value, *rhs).into()
    }
}
impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Dl::Kind: marker::Div,
    Dr::Kind: marker::Div,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
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
        Div::div(self.value, rhs.value).into()
    }
}
impl<'l, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<Quantity<Dr, Ur, V>>
    for &'l DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
    Dl::Kind: marker::Div,
    Dr::Kind: marker::Div,
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
        Div::div(self.value, rhs.value).into()
    }
}
impl<'l, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<Quantity<Dr, Ur, V>>
    for &'l mut DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
    Dl::Kind: marker::Div,
    Dr::Kind: marker::Div,
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
        Div::div(self.value, rhs.value).into()
    }
}
impl<'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<&'r Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
    Dl::Kind: marker::Div,
    Dr::Kind: marker::Div,
    Dl::L: Sub<Dr::L>,
    Dl::M: Sub<Dr::M>,
    Dl::T: Sub<Dr::T>,
    Dl::I: Sub<Dr::I>,
    Dl::Th: Sub<Dr::Th>,
    Dl::N: Sub<Dr::N>,
    Dl::J: Sub<Dr::J>,
{
    type Output = DimVector3<DiffDimension<Dl, Dr>, Ul, V>;
    fn div(self, rhs: &Quantity<Dr, Ur, V>) -> Self::Output {
        Div::div(self.value, rhs.value).into()
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<&'r Quantity<Dr, Ur, V>>
    for &'l DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
    Dl::Kind: marker::Div,
    Dr::Kind: marker::Div,
    Dl::L: Sub<Dr::L>,
    Dl::M: Sub<Dr::M>,
    Dl::T: Sub<Dr::T>,
    Dl::I: Sub<Dr::I>,
    Dl::Th: Sub<Dr::Th>,
    Dl::N: Sub<Dr::N>,
    Dl::J: Sub<Dr::J>,
{
    type Output = DimVector3<DiffDimension<Dl, Dr>, Ul, V>;
    fn div(self, rhs: &Quantity<Dr, Ur, V>) -> Self::Output {
        Div::div(self.value, rhs.value).into()
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<&'r Quantity<Dr, Ur, V>>
    for &'l mut DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
    Dl::Kind: marker::Div,
    Dr::Kind: marker::Div,
    Dl::L: Sub<Dr::L>,
    Dl::M: Sub<Dr::M>,
    Dl::T: Sub<Dr::T>,
    Dl::I: Sub<Dr::I>,
    Dl::Th: Sub<Dr::Th>,
    Dl::N: Sub<Dr::N>,
    Dl::J: Sub<Dr::J>,
{
    type Output = DimVector3<DiffDimension<Dl, Dr>, Ul, V>;
    fn div(self, rhs: &Quantity<Dr, Ur, V>) -> Self::Output {
        Div::div(self.value, rhs.value).into()
    }
}
impl<'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<&'r mut Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
    Dl::Kind: marker::Div,
    Dr::Kind: marker::Div,
    Dl::L: Sub<Dr::L>,
    Dl::M: Sub<Dr::M>,
    Dl::T: Sub<Dr::T>,
    Dl::I: Sub<Dr::I>,
    Dl::Th: Sub<Dr::Th>,
    Dl::N: Sub<Dr::N>,
    Dl::J: Sub<Dr::J>,
{
    type Output = DimVector3<DiffDimension<Dl, Dr>, Ul, V>;
    fn div(self, rhs: &mut Quantity<Dr, Ur, V>) -> Self::Output {
        Div::div(self.value, rhs.value).into()
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<&'r mut Quantity<Dr, Ur, V>>
    for &'l DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
    Dl::Kind: marker::Div,
    Dr::Kind: marker::Div,
    Dl::L: Sub<Dr::L>,
    Dl::M: Sub<Dr::M>,
    Dl::T: Sub<Dr::T>,
    Dl::I: Sub<Dr::I>,
    Dl::Th: Sub<Dr::Th>,
    Dl::N: Sub<Dr::N>,
    Dl::J: Sub<Dr::J>,
{
    type Output = DimVector3<DiffDimension<Dl, Dr>, Ul, V>;
    fn div(self, rhs: &mut Quantity<Dr, Ur, V>) -> Self::Output {
        Div::div(self.value, rhs.value).into()
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<&'r mut Quantity<Dr, Ur, V>>
    for &'l mut DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
    Dl::Kind: marker::Div,
    Dr::Kind: marker::Div,
    Dl::L: Sub<Dr::L>,
    Dl::M: Sub<Dr::M>,
    Dl::T: Sub<Dr::T>,
    Dl::I: Sub<Dr::I>,
    Dl::Th: Sub<Dr::Th>,
    Dl::N: Sub<Dr::N>,
    Dl::J: Sub<Dr::J>,
{
    type Output = DimVector3<DiffDimension<Dl, Dr>, Ul, V>;
    fn div(self, rhs: &mut Quantity<Dr, Ur, V>) -> Self::Output {
        Div::div(self.value, rhs.value).into()
    }
}
