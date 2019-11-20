use crate::{
    util::{marker, Dimension, MyQuantity, Numeric, ISQ},
    vector3,
    vectors::*,
};

use core::ops::{Div, Sub};

use typenum::operator_aliases::Diff;

pub type DiffDimension<Dl, Dr> = ISQ<
    Diff<<Dl as Dimension>::L, <Dr as Dimension>::L>,
    Diff<<Dl as Dimension>::M, <Dr as Dimension>::M>,
    Diff<<Dl as Dimension>::T, <Dr as Dimension>::T>,
    Diff<<Dl as Dimension>::I, <Dr as Dimension>::I>,
    Diff<<Dl as Dimension>::Th, <Dr as Dimension>::Th>,
    Diff<<Dl as Dimension>::N, <Dr as Dimension>::N>,
    Diff<<Dl as Dimension>::J, <Dr as Dimension>::J>,
>;

impl<Dl: ?Sized, Dr: ?Sized> Div<MyQuantity<Dr>> for MyVector3<Dl>
where
    Dl: Dimension,
    Dr: Dimension,
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
    type Output = MyVector3<DiffDimension<Dl, Dr>>;
    fn div(self, rhs: MyQuantity<Dr>) -> Self::Output {
        vector3!(self.value / rhs.value)
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized> Div<&'r MyQuantity<Dr>> for &'l MyVector3<Dl>
where
    Dl: Dimension,
    Dr: Dimension,
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
    type Output = MyVector3<DiffDimension<Dl, Dr>>;
    fn div(self, rhs: &MyQuantity<Dr>) -> Self::Output {
        vector3!(self.value / rhs.value)
    }
}
impl<'r, Dl: ?Sized, Dr: ?Sized> Div<&'r MyQuantity<Dr>> for MyVector3<Dl>
where
    Dl: Dimension,
    Dr: Dimension,
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
    type Output = MyVector3<DiffDimension<Dl, Dr>>;
    fn div(self, rhs: &MyQuantity<Dr>) -> Self::Output {
        vector3!(self.value / rhs.value)
    }
}
impl<D: ?Sized> Div<Numeric> for MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::Div,
{
    type Output = Self;
    fn div(self, rhs: Numeric) -> Self::Output {
        vector3!(self.value / rhs)
    }
}
