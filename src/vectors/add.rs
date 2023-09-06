use super::DimVector3;

use std::ops::{Add, Sub};

use nalgebra::{base::Scalar, ClosedAdd, ClosedSub};

impl<D: ?Sized, U: ?Sized, V> Add<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedAdd,
{
    type Output = DimVector3<D, U, V>;
    fn add(self, rhs: DimVector3<D, U, V>) -> Self::Output {
        Add::add(self.value, rhs.value).into()
    }
}

impl<D: ?Sized, U: ?Sized, V> Sub<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedSub,
{
    type Output = DimVector3<D, U, V>;
    fn sub(self, rhs: DimVector3<D, U, V>) -> Self::Output {
        Sub::sub(self.value, rhs.value).into()
    }
}
