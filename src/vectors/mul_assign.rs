use super::DimVector3;

use crate::units::{Conversion, Dimension, Num, Quantity, Units};

use std::ops::{Add, DivAssign, MulAssign, Sub};

use nalgebra::{base::Scalar, ClosedDiv, ClosedMul};

impl<D: ?Sized, U: ?Sized, V> MulAssign<V> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedMul,
{
    fn mul_assign(&mut self, rhs: V) {
        MulAssign::mul_assign(&mut self.value, rhs)
    }
}

impl<D: ?Sized, U: ?Sized, V> DivAssign<V> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedDiv,
{
    fn div_assign(&mut self, rhs: V) {
        DivAssign::div_assign(&mut self.value, rhs)
    }
}

impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> MulAssign<Quantity<Dr, Ur, V>>
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
    V: Num + Conversion<V> + Scalar + Copy + ClosedMul,
{
    fn mul_assign(&mut self, rhs: Quantity<Dr, Ur, V>) {
        MulAssign::mul_assign(&mut self.value, rhs.value)
    }
}
impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> DivAssign<Quantity<Dr, Ur, V>>
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
    V: Num + Conversion<V> + Scalar + Copy + ClosedDiv,
{
    fn div_assign(&mut self, rhs: Quantity<Dr, Ur, V>) {
        DivAssign::div_assign(&mut self.value, rhs.value)
    }
}
