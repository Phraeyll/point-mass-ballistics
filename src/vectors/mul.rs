use crate::{
    units::{marker, Conversion, Dimension, Quantity, Units},
    vectors::*,
};

use core::ops::{Add, Div, Mul, Sub};

use nalgebra::{base::Scalar, ClosedDiv, ClosedMul};
use num_traits::Num;

impl<D: ?Sized, U: ?Sized, V> Mul<V> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedMul,
{
    type Output = DimVector3<D, U, V>;
    fn mul(self, rhs: V) -> Self::Output {
        Mul::mul(self.value, rhs).into()
    }
}

impl<D: ?Sized, U: ?Sized, V> Div<V> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedDiv,
{
    type Output = DimVector3<D, U, V>;
    fn div(self, rhs: V) -> Self::Output {
        Div::div(self.value, rhs).into()
    }
}

impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Dl::Kind: marker::Mul,
    Dr::Kind: marker::Mul,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedMul,
    Dl::L: Add<Dr::L>,
    Dl::M: Add<Dr::M>,
    Dl::T: Add<Dr::T>,
    Dl::I: Add<Dr::I>,
    Dl::Th: Add<Dr::Th>,
    Dl::N: Add<Dr::N>,
    Dl::J: Add<Dr::J>,
{
    type Output = DimVector3<SumDimension<Dl, Dr>, Ul, V>;
    fn mul(self, rhs: Quantity<Dr, Ur, V>) -> Self::Output {
        Mul::mul(self.value, rhs.value).into()
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
