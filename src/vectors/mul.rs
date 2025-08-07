use super::{DiffDimension, DimVector3, SumDimension};

use crate::units::{Conversion, Dimension, Num, Quantity, Units};

use std::ops::{Add, Div, Mul, Sub};

use nalgebra::{ClosedDivAssign, ClosedMulAssign, base::Scalar};

impl<D: ?Sized, U: ?Sized, V> Mul<V> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedMulAssign,
{
    type Output = DimVector3<D, U, V>;
    fn mul(self, rhs: V) -> Self::Output {
        Mul::mul(self.value, rhs).into()
    }
}

impl<D: ?Sized, U: ?Sized, V> Div<V> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedDivAssign,
{
    type Output = DimVector3<D, U, V>;
    fn div(self, rhs: V) -> Self::Output {
        Div::div(self.value, rhs).into()
    }
}

impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Mul<Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension<
            L: Add<Dr::L>,
            M: Add<Dr::M>,
            T: Add<Dr::T>,
            I: Add<Dr::I>,
            Th: Add<Dr::Th>,
            N: Add<Dr::N>,
            J: Add<Dr::J>,
        >,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedMulAssign,
{
    type Output = DimVector3<SumDimension<Dl, Dr>, Ul, V>;
    fn mul(self, rhs: Quantity<Dr, Ur, V>) -> Self::Output {
        Mul::mul(self.value, rhs.value).into()
    }
}

impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Div<Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension<
            L: Sub<Dr::L>,
            M: Sub<Dr::M>,
            T: Sub<Dr::T>,
            I: Sub<Dr::I>,
            Th: Sub<Dr::Th>,
            N: Sub<Dr::N>,
            J: Sub<Dr::J>,
        >,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy + ClosedDivAssign,
{
    type Output = DimVector3<DiffDimension<Dl, Dr>, Ul, V>;
    fn div(self, rhs: Quantity<Dr, Ur, V>) -> Self::Output {
        Div::div(self.value, rhs.value).into()
    }
}
