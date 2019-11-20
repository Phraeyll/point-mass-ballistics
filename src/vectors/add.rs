use crate::{
    util::{marker, Dimension},
    vector3,
    vectors::MyVector3,
};

use core::ops::Add;

impl<D: ?Sized> Add for MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::Add,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        vector3!(self.value + rhs.value)
    }
}
impl<'l, 'r, D: ?Sized> Add<&'r Self> for &'l MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::Add,
{
    type Output = MyVector3<D>;
    fn add(self, rhs: &Self) -> Self::Output {
        vector3!(self.value + rhs.value)
    }
}
impl<'r, D: ?Sized> Add<&'r Self> for MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::Add,
{
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        vector3!(self.value + rhs.value)
    }
}
impl<'l, D: ?Sized> Add<MyVector3<D>> for &'l MyVector3<D>
where
    D: Dimension,
    D::Kind: marker::Add,
{
    type Output = MyVector3<D>;
    fn add(self, rhs: MyVector3<D>) -> Self::Output {
        vector3!(self.value + rhs.value)
    }
}
