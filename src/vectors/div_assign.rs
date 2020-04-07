use crate::{
    util::{marker, Dimension, Quantity},
    vectors::*,
};

use core::ops::{DivAssign, Sub};

use nalgebra::{base::Scalar, ClosedDiv};
use num_traits::Num;

impl<D: ?Sized, U: ?Sized, V> DivAssign<V> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedDiv,
{
    fn div_assign(&mut self, rhs: V) {
        DivAssign::div_assign(&mut self.value, rhs)
    }
}
impl<D: ?Sized, U: ?Sized, V> DivAssign<&V> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedDiv,
{
    fn div_assign(&mut self, rhs: &V) {
        DivAssign::div_assign(&mut self.value, *rhs)
    }
}
impl<D: ?Sized, U: ?Sized, V> DivAssign<&mut V> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedDiv,
{
    fn div_assign(&mut self, rhs: &mut V) {
        DivAssign::div_assign(&mut self.value, *rhs)
    }
}
impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> DivAssign<Quantity<Dr, Ur, V>>
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
    fn div_assign(&mut self, rhs: Quantity<Dr, Ur, V>) {
        DivAssign::div_assign(&mut self.value, rhs.value)
    }
}
impl<'l, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> DivAssign<Quantity<Dr, Ur, V>>
    for &'l mut DimVector3<Dl, Ul, V>
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
    fn div_assign(&mut self, rhs: Quantity<Dr, Ur, V>) {
        DivAssign::div_assign(&mut self.value, rhs.value)
    }
}
impl<'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> DivAssign<&'r Quantity<Dr, Ur, V>>
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
    fn div_assign(&mut self, rhs: &Quantity<Dr, Ur, V>) {
        DivAssign::div_assign(&mut self.value, rhs.value)
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> DivAssign<&'r Quantity<Dr, Ur, V>>
    for &'l mut DimVector3<Dl, Ul, V>
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
    fn div_assign(&mut self, rhs: &Quantity<Dr, Ur, V>) {
        DivAssign::div_assign(&mut self.value, rhs.value)
    }
}
impl<'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> DivAssign<&'r mut Quantity<Dr, Ur, V>>
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
    fn div_assign(&mut self, rhs: &mut Quantity<Dr, Ur, V>) {
        DivAssign::div_assign(&mut self.value, rhs.value)
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V>
    DivAssign<&'r mut Quantity<Dr, Ur, V>> for &'l mut DimVector3<Dl, Ul, V>
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
    fn div_assign(&mut self, rhs: &mut Quantity<Dr, Ur, V>) {
        DivAssign::div_assign(&mut self.value, rhs.value)
    }
}
