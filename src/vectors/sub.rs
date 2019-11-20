use crate::{
    util::{marker, Dimension},
    vector3,
    vectors::MyVector3,
};

use core::ops::Sub;

impl<D: ?Sized> Sub for MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::Sub,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        vector3!(self.value - rhs.value)
    }
}
impl<'l, 'r, D: ?Sized> Sub<&'r Self> for &'l MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::Sub,
{
    type Output = MyVector3<D>;
    fn sub(self, rhs: &Self) -> Self::Output {
        vector3!(self.value - rhs.value)
    }
}
impl<'r, D: ?Sized> Sub<&'r Self> for MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::Sub,
{
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        vector3!(self.value - rhs.value)
    }
}
impl<'l, D: ?Sized> Sub<MyVector3<D>> for &'l MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::Sub,
{
    type Output = MyVector3<D>;
    fn sub(self, rhs: MyVector3<D>) -> Self::Output {
        vector3!(self.value - rhs.value)
    }
}
