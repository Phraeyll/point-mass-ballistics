use crate::{
    util::{marker, Conversion, Dimension, Quantity, Units},
    vectors::*,
};

use core::ops::{Add, Mul};

use nalgebra::{base::Scalar, ClosedMul};
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
impl<'r, D: ?Sized, U: ?Sized, V> Mul<&'r V> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedMul,
{
    type Output = DimVector3<D, U, V>;
    fn mul(self, rhs: &V) -> Self::Output {
        Mul::mul(self.value, *rhs).into()
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> Mul<&'r mut V> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedMul,
{
    type Output = DimVector3<D, U, V>;
    fn mul(self, rhs: &mut V) -> Self::Output {
        Mul::mul(self.value, *rhs).into()
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
impl<'l, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<Quantity<Dr, Ur, V>>
    for &'l DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedMul,
    Dl::Kind: marker::Mul,
    Dr::Kind: marker::Mul,
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
impl<'l, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<Quantity<Dr, Ur, V>>
    for &'l mut DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedMul,
    Dl::Kind: marker::Mul,
    Dr::Kind: marker::Mul,
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
impl<'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<&'r Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedMul,
    Dl::Kind: marker::Mul,
    Dr::Kind: marker::Mul,
    Dl::L: Add<Dr::L>,
    Dl::M: Add<Dr::M>,
    Dl::T: Add<Dr::T>,
    Dl::I: Add<Dr::I>,
    Dl::Th: Add<Dr::Th>,
    Dl::N: Add<Dr::N>,
    Dl::J: Add<Dr::J>,
{
    type Output = DimVector3<SumDimension<Dl, Dr>, Ul, V>;
    fn mul(self, rhs: &Quantity<Dr, Ur, V>) -> Self::Output {
        Mul::mul(self.value, rhs.value).into()
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<&'r Quantity<Dr, Ur, V>>
    for &'l DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedMul,
    Dl::Kind: marker::Mul,
    Dr::Kind: marker::Mul,
    Dl::L: Add<Dr::L>,
    Dl::M: Add<Dr::M>,
    Dl::T: Add<Dr::T>,
    Dl::I: Add<Dr::I>,
    Dl::Th: Add<Dr::Th>,
    Dl::N: Add<Dr::N>,
    Dl::J: Add<Dr::J>,
{
    type Output = DimVector3<SumDimension<Dl, Dr>, Ul, V>;
    fn mul(self, rhs: &Quantity<Dr, Ur, V>) -> Self::Output {
        Mul::mul(self.value, rhs.value).into()
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<&'r Quantity<Dr, Ur, V>>
    for &'l mut DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedMul,
    Dl::Kind: marker::Mul,
    Dr::Kind: marker::Mul,
    Dl::L: Add<Dr::L>,
    Dl::M: Add<Dr::M>,
    Dl::T: Add<Dr::T>,
    Dl::I: Add<Dr::I>,
    Dl::Th: Add<Dr::Th>,
    Dl::N: Add<Dr::N>,
    Dl::J: Add<Dr::J>,
{
    type Output = DimVector3<SumDimension<Dl, Dr>, Ul, V>;
    fn mul(self, rhs: &Quantity<Dr, Ur, V>) -> Self::Output {
        Mul::mul(self.value, rhs.value).into()
    }
}
impl<'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<&'r mut Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedMul,
    Dl::Kind: marker::Mul,
    Dr::Kind: marker::Mul,
    Dl::L: Add<Dr::L>,
    Dl::M: Add<Dr::M>,
    Dl::T: Add<Dr::T>,
    Dl::I: Add<Dr::I>,
    Dl::Th: Add<Dr::Th>,
    Dl::N: Add<Dr::N>,
    Dl::J: Add<Dr::J>,
{
    type Output = DimVector3<SumDimension<Dl, Dr>, Ul, V>;
    fn mul(self, rhs: &mut Quantity<Dr, Ur, V>) -> Self::Output {
        Mul::mul(self.value, rhs.value).into()
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<&'r mut Quantity<Dr, Ur, V>>
    for &'l DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedMul,
    Dl::Kind: marker::Mul,
    Dr::Kind: marker::Mul,
    Dl::L: Add<Dr::L>,
    Dl::M: Add<Dr::M>,
    Dl::T: Add<Dr::T>,
    Dl::I: Add<Dr::I>,
    Dl::Th: Add<Dr::Th>,
    Dl::N: Add<Dr::N>,
    Dl::J: Add<Dr::J>,
{
    type Output = DimVector3<SumDimension<Dl, Dr>, Ul, V>;
    fn mul(self, rhs: &mut Quantity<Dr, Ur, V>) -> Self::Output {
        Mul::mul(self.value, rhs.value).into()
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<&'r mut Quantity<Dr, Ur, V>>
    for &'l mut DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedMul,
    Dl::Kind: marker::Mul,
    Dr::Kind: marker::Mul,
    Dl::L: Add<Dr::L>,
    Dl::M: Add<Dr::M>,
    Dl::T: Add<Dr::T>,
    Dl::I: Add<Dr::I>,
    Dl::Th: Add<Dr::Th>,
    Dl::N: Add<Dr::N>,
    Dl::J: Add<Dr::J>,
{
    type Output = DimVector3<SumDimension<Dl, Dr>, Ul, V>;
    fn mul(self, rhs: &mut Quantity<Dr, Ur, V>) -> Self::Output {
        Mul::mul(self.value, rhs.value).into()
    }
}
