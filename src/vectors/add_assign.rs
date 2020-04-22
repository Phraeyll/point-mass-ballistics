use crate::vectors::DimVector3;

use core::ops::{AddAssign, SubAssign};

use nalgebra::{base::Scalar, ClosedAdd, ClosedSub};

impl<D: ?Sized, U: ?Sized, V> AddAssign<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedAdd,
{
    fn add_assign(&mut self, rhs: DimVector3<D, U, V>) {
        AddAssign::add_assign(&mut self.value, rhs.value)
    }
}

impl<D: ?Sized, U: ?Sized, V> SubAssign<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedSub,
{
    fn sub_assign(&mut self, rhs: DimVector3<D, U, V>) {
        SubAssign::sub_assign(&mut self.value, rhs.value)
    }
}
