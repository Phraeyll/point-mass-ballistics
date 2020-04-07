use crate::vectors::DimVector3;

use core::ops::SubAssign;

use nalgebra::{base::Scalar, ClosedSub};

impl<D: ?Sized, U: ?Sized, V> SubAssign<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedSub,
{
    fn sub_assign(&mut self, rhs: DimVector3<D, U, V>) {
        SubAssign::sub_assign(&mut self.value, rhs.value)
    }
}
impl<'l, D: ?Sized, U: ?Sized, V> SubAssign<DimVector3<D, U, V>> for &'l mut DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedSub,
{
    fn sub_assign(&mut self, rhs: DimVector3<D, U, V>) {
        SubAssign::sub_assign(&mut self.value, rhs.value)
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> SubAssign<&'r DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedSub,
{
    fn sub_assign(&mut self, rhs: &DimVector3<D, U, V>) {
        SubAssign::sub_assign(&mut self.value, rhs.value)
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> SubAssign<&'r DimVector3<D, U, V>>
    for &'l mut DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedSub,
{
    fn sub_assign(&mut self, rhs: &DimVector3<D, U, V>) {
        SubAssign::sub_assign(&mut self.value, rhs.value)
    }
}
impl<'r, D: ?Sized, U: ?Sized, V> SubAssign<&'r mut DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedSub,
{
    fn sub_assign(&mut self, rhs: &mut DimVector3<D, U, V>) {
        SubAssign::sub_assign(&mut self.value, rhs.value)
    }
}
impl<'l, 'r, D: ?Sized, U: ?Sized, V> SubAssign<&'r mut DimVector3<D, U, V>>
    for &'l mut DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedSub,
{
    fn sub_assign(&mut self, rhs: &mut DimVector3<D, U, V>) {
        SubAssign::sub_assign(&mut self.value, rhs.value)
    }
}
