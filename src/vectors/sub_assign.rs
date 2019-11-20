use crate::{
    util::{marker, Dimension},
    vectors::MyVector3,
};

use core::ops::SubAssign;

impl<D: ?Sized> SubAssign for MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value
    }
}
impl<'l, 'r, D: ?Sized> SubAssign<&'r Self> for &'l mut MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::SubAssign,
{
    fn sub_assign(&mut self, rhs: &Self) {
        self.value -= rhs.value
    }
}
impl<'r, D: ?Sized> SubAssign<&'r Self> for MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::SubAssign,
{
    fn sub_assign(&mut self, rhs: &Self) {
        self.value -= rhs.value
    }
}
impl<'l, D: ?Sized> SubAssign<MyVector3<D>> for &'l mut MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::SubAssign,
{
    fn sub_assign(&mut self, rhs: MyVector3<D>) {
        self.value -= rhs.value
    }
}
