use crate::vectors::DimVector3;

use core::ops::AddAssign;

use nalgebra::{base::Scalar, ClosedAdd};

impl<D: ?Sized, U: ?Sized, V> AddAssign<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedAdd,
{
    fn add_assign(&mut self, rhs: DimVector3<D, U, V>) {
        AddAssign::add_assign(&mut self.value, rhs.value)
    }
}
impl<'l, D: ?Sized, U: ?Sized, V> AddAssign<DimVector3<D, U, V>> for &'l mut DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedAdd,
{
    fn add_assign(&mut self, rhs: DimVector3<D, U, V>) {
        AddAssign::add_assign(&mut self.value, rhs.value)
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> AddAssign<&'r DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedAdd,
{
    fn add_assign(&mut self, rhs: &DimVector3<D, U, V>) {
        AddAssign::add_assign(&mut self.value, rhs.value)
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> AddAssign<&'r DimVector3<D, U, V>>
    for &'l mut DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedAdd,
{
    fn add_assign(&mut self, rhs: &DimVector3<D, U, V>) {
        AddAssign::add_assign(&mut self.value, rhs.value)
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> AddAssign<&'r mut DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedAdd,
{
    fn add_assign(&mut self, rhs: &mut DimVector3<D, U, V>) {
        AddAssign::add_assign(&mut self.value, rhs.value)
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> AddAssign<&'r mut DimVector3<D, U, V>>
    for &'l mut DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedAdd,
{
    fn add_assign(&mut self, rhs: &mut DimVector3<D, U, V>) {
        AddAssign::add_assign(&mut self.value, rhs.value)
    }
}
