use super::DimVector3;

use std::cmp::{Ordering, PartialEq, PartialOrd};

use nalgebra::{base::Scalar, ClosedAddAssign};

impl<D: ?Sized, U: ?Sized, V> PartialEq<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + ClosedAddAssign,
{
    fn eq(&self, rhs: &DimVector3<D, U, V>) -> bool {
        PartialEq::eq(&self.value, &rhs.value)
    }
}

impl<D: ?Sized, U: ?Sized, V> PartialOrd<DimVector3<D, U, V>> for DimVector3<D, U, V>
where
    V: Scalar + Copy + PartialOrd + ClosedAddAssign,
{
    fn partial_cmp(&self, rhs: &DimVector3<D, U, V>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.value, &rhs.value)
    }
}
