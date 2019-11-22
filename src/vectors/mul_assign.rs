use crate::{
    util::{marker, Dimension, Quantity},
    vectors::*,
};

use core::ops::{Add, MulAssign};

use alga::general::ClosedMul;
use nalgebra::base::Scalar;
use num_traits::Num;

impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> MulAssign<Quantity<Dr, Ur, V>>
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
    fn mul_assign(&mut self, rhs: Quantity<Dr, Ur, V>) {
        self.value *= rhs.value
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> MulAssign<&'r Quantity<Dr, Ur, V>>
    for &'l mut DimVector3<Dl, Ul, V>
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
    fn mul_assign(&mut self, rhs: &Quantity<Dr, Ur, V>) {
        self.value *= rhs.value
    }
}
impl<'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> MulAssign<&'r Quantity<Dr, Ur, V>>
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
    fn mul_assign(&mut self, rhs: &Quantity<Dr, Ur, V>) {
        self.value *= rhs.value
    }
}
impl<D: ?Sized, U: ?Sized, V> MulAssign<V> for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + ClosedMul,
    D::Kind: marker::MulAssign,
{
    fn mul_assign(&mut self, rhs: V) {
        self.value *= rhs
    }
}
