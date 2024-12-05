use super::DimVector3;

use std::ops::{AddAssign, SubAssign};

use nalgebra::{base::Scalar, ClosedAddAssign, ClosedSubAssign};

impl<D: ?Sized, U: ?Sized, V> AddAssign<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedAddAssign,
{
    fn add_assign(&mut self, rhs: DimVector3<D, U, V>) {
        AddAssign::add_assign(&mut self.value, rhs.value)
    }
}

impl<D: ?Sized, U: ?Sized, V> SubAssign<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedSubAssign,
{
    fn sub_assign(&mut self, rhs: DimVector3<D, U, V>) {
        SubAssign::sub_assign(&mut self.value, rhs.value)
    }
}
