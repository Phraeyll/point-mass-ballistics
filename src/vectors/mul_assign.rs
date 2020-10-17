use crate::{
    units::{Conversion, Dimension, Quantity, Units},
    vectors::DimVector3,
};

use core::ops::{Add, DivAssign, MulAssign, Sub};

use nalgebra::{base::Scalar, ClosedDiv, ClosedMul};
use num_traits::Num;

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
    Dl: Dimension,
    Dr: Dimension,
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
    fn mul_assign(&mut self, rhs: Quantity<Dr, Ur, V>) {
        MulAssign::mul_assign(&mut self.value, rhs.value)
    }
}
impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> DivAssign<Quantity<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
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
    fn div_assign(&mut self, rhs: Quantity<Dr, Ur, V>) {
        DivAssign::div_assign(&mut self.value, rhs.value)
    }
}
