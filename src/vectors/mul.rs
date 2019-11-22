use crate::{
    util::{marker, Conversion, Dimension, Quantity, Units},
    vector3,
    vectors::*,
};

use core::ops::{Add, Mul};

use alga::general::ClosedMul;
use nalgebra::base::Scalar;
use num_traits::Num;

impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Dl::Kind: marker::Mul,
    Dr::Kind: marker::Mul,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedMul,
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
        vector3!(self.value * rhs.value)
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<&'r Quantity<Dr, Ur, V>>
    for &'l DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedMul,
    Dl::Kind: marker::Mul,
    Dr::Kind: marker::Mul,
    Dl::L: Add<Dr::L> + 'r,
    Dl::M: Add<Dr::M> + 'r,
    Dl::T: Add<Dr::T> + 'r,
    Dl::I: Add<Dr::I> + 'r,
    Dl::Th: Add<Dr::Th> + 'r,
    Dl::N: Add<Dr::N> + 'r,
    Dl::J: Add<Dr::J> + 'r,
{
    type Output = DimVector3<SumDimension<Dl, Dr>, Ul, V>;
    fn mul(self, rhs: &Quantity<Dr, Ur, V>) -> Self::Output {
        vector3!(self.value * rhs.value)
    }
}
impl<'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<&'r Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedMul,
    Dl::Kind: marker::Mul,
    Dr::Kind: marker::Mul,
    Dl::L: Add<Dr::L> + 'r,
    Dl::M: Add<Dr::M> + 'r,
    Dl::T: Add<Dr::T> + 'r,
    Dl::I: Add<Dr::I> + 'r,
    Dl::Th: Add<Dr::Th> + 'r,
    Dl::N: Add<Dr::N> + 'r,
    Dl::J: Add<Dr::J> + 'r,
{
    type Output = DimVector3<SumDimension<Dl, Dr>, Ul, V>;
    fn mul(self, rhs: &Quantity<Dr, Ur, V>) -> Self::Output {
        vector3!(self.value * rhs.value)
    }
}
impl<D: ?Sized, U: ?Sized, V> Mul<V> for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedMul,
    D::Kind: marker::Mul,
{
    type Output = Self;
    fn mul(self, rhs: V) -> Self::Output {
        vector3!(self.value * rhs)
    }
}
