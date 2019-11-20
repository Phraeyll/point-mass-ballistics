use crate::{
    util::{marker, Dimension},
    vectors::MyVector3,
};

use core::ops::AddAssign;

impl<D: ?Sized> AddAssign for MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}
impl<'l, 'r, D: ?Sized> AddAssign<&'r Self> for &'l mut MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::AddAssign,
{
    fn add_assign(&mut self, rhs: &Self) {
        self.value += rhs.value
    }
}
impl<'r, D: ?Sized> AddAssign<&'r Self> for MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::AddAssign,
{
    fn add_assign(&mut self, rhs: &Self) {
        self.value += rhs.value
    }
}
impl<'l, D: ?Sized> AddAssign<MyVector3<D>> for &'l mut MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::AddAssign,
{
    fn add_assign(&mut self, rhs: MyVector3<D>) {
        self.value += rhs.value
    }
}
