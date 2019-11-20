use crate::{
    util::{marker, Dimension, MyQuantity, Numeric, ISQ},
    vector3,
    vectors::*,
};

use core::ops::{Add, Mul};

use typenum::operator_aliases::Sum;

pub type SumDimension<Dl, Dr> = ISQ<
    Sum<<Dl as Dimension>::L, <Dr as Dimension>::L>,
    Sum<<Dl as Dimension>::M, <Dr as Dimension>::M>,
    Sum<<Dl as Dimension>::T, <Dr as Dimension>::T>,
    Sum<<Dl as Dimension>::I, <Dr as Dimension>::I>,
    Sum<<Dl as Dimension>::Th, <Dr as Dimension>::Th>,
    Sum<<Dl as Dimension>::N, <Dr as Dimension>::N>,
    Sum<<Dl as Dimension>::J, <Dr as Dimension>::J>,
>;

impl<Dl: ?Sized, Dr: ?Sized> Mul<MyQuantity<Dr>> for MyVector3<Dl>
where
    Dl: Dimension,
    Dr: Dimension,
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
    type Output = MyVector3<SumDimension<Dl, Dr>>;
    fn mul(self, rhs: MyQuantity<Dr>) -> Self::Output {
        vector3!(self.value * rhs.value)
    }
}
impl<'l, 'r, Dl: ?Sized, Dr: ?Sized> Mul<&'r MyQuantity<Dr>> for &'l MyVector3<Dl>
where
    Dl: Dimension,
    Dr: Dimension,
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
    type Output = MyVector3<SumDimension<Dl, Dr>>;
    fn mul(self, rhs: &MyQuantity<Dr>) -> Self::Output {
        vector3!(self.value * rhs.value)
    }
}
impl<'r, Dl: ?Sized, Dr: ?Sized> Mul<&'r MyQuantity<Dr>> for MyVector3<Dl>
where
    Dl: Dimension,
    Dr: Dimension,
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
    type Output = MyVector3<SumDimension<Dl, Dr>>;
    fn mul(self, rhs: &MyQuantity<Dr>) -> Self::Output {
        vector3!(self.value * rhs.value)
    }
}
impl<D: ?Sized> Mul<Numeric> for MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::Mul,
{
    type Output = Self;
    fn mul(self, rhs: Numeric) -> Self::Output {
        vector3!(self.value * rhs)
    }
}
