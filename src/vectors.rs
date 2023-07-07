pub use self::{add::*, add_assign::*, mul::*, mul_assign::*};
use crate::{
    units::{
        angle::radian,
        quantity,
        typenum::operator_aliases::{Diff, Sum},
        Angle, ConstZero, Conversion, Dimension, MyUnits, Num, Quantity, Units, ISQ,
    },
    Numeric,
};

use core::ops::Add;
use std::{fmt, marker::PhantomData};

use nalgebra::{
    base::Scalar, ClosedAdd, ClosedMul, ClosedSub, Rotation3, SimdComplexField, Vector3,
};

mod add;
mod add_assign;
mod mul;
mod mul_assign;

pub type MyVector3<D> = DimVector3<D, MyUnits, Numeric>;

pub struct DimVector3<D: ?Sized, U: ?Sized, V>
where
    V: Scalar,
{
    dimension: PhantomData<D>,
    units: PhantomData<U>,
    value: Vector3<V>,
}

impl<D: ?Sized, U: ?Sized, V> From<Vector3<V>> for DimVector3<D, U, V>
where
    V: Scalar,
{
    fn from(other: Vector3<V>) -> Self {
        Self {
            dimension: PhantomData,
            units: PhantomData,
            value: other,
        }
    }
}

impl<D: ?Sized, U: ?Sized, V> From<DimVector3<D, U, V>> for Vector3<V>
where
    V: Scalar,
{
    fn from(other: DimVector3<D, U, V>) -> Self {
        other.value
    }
}

impl<D: ?Sized, U: ?Sized, V> ConstZero for DimVector3<D, U, V>
where
    V: Scalar + ConstZero,
{
    const ZERO: Self = Self {
        dimension: PhantomData,
        units: PhantomData,
        value: Vector3::new(V::ZERO, V::ZERO, V::ZERO),
    };
}

pub type SumDimension<Dl, Dr> = ISQ<
    Sum<<Dl as Dimension>::L, <Dr as Dimension>::L>,
    Sum<<Dl as Dimension>::M, <Dr as Dimension>::M>,
    Sum<<Dl as Dimension>::T, <Dr as Dimension>::T>,
    Sum<<Dl as Dimension>::I, <Dr as Dimension>::I>,
    Sum<<Dl as Dimension>::Th, <Dr as Dimension>::Th>,
    Sum<<Dl as Dimension>::N, <Dr as Dimension>::N>,
    Sum<<Dl as Dimension>::J, <Dr as Dimension>::J>,
>;

pub type DiffDimension<Dl, Dr> = ISQ<
    Diff<<Dl as Dimension>::L, <Dr as Dimension>::L>,
    Diff<<Dl as Dimension>::M, <Dr as Dimension>::M>,
    Diff<<Dl as Dimension>::T, <Dr as Dimension>::T>,
    Diff<<Dl as Dimension>::I, <Dr as Dimension>::I>,
    Diff<<Dl as Dimension>::Th, <Dr as Dimension>::Th>,
    Diff<<Dl as Dimension>::N, <Dr as Dimension>::N>,
    Diff<<Dl as Dimension>::J, <Dr as Dimension>::J>,
>;

impl<D: ?Sized, U: ?Sized, V> fmt::Debug for DimVector3<D, U, V>
where
    V: Scalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<D: ?Sized, U: ?Sized, V> Copy for DimVector3<D, U, V> where V: Scalar + Copy {}

impl<D: ?Sized, U: ?Sized, V> Clone for DimVector3<D, U, V>
where
    V: Scalar,
{
    fn clone(&self) -> Self {
        self.value.clone().into()
    }
}

pub trait Cross<Rhs = Self> {
    type Output;
    fn cross(&self, rhs: Rhs) -> Self::Output;
}

pub trait Norm {
    type Output;
    fn norm(&self) -> Self::Output;
}

impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Cross<&DimVector3<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Conversion<V> + Scalar + Copy + ClosedAdd + ClosedMul + ClosedSub,
    Dl::L: Add<Dr::L>,
    Dl::M: Add<Dr::M>,
    Dl::T: Add<Dr::T>,
    Dl::I: Add<Dr::I>,
    Dl::Th: Add<Dr::Th>,
    Dl::N: Add<Dr::N>,
    Dl::J: Add<Dr::J>,
{
    type Output = DimVector3<SumDimension<Dl, Dr>, Ul, V>;
    fn cross(&self, rhs: &DimVector3<Dr, Ur, V>) -> Self::Output {
        self.value.cross(&rhs.value).into()
    }
}

impl<D: ?Sized, U: ?Sized, V> Norm for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    U: Units<<V as SimdComplexField>::SimdRealField>,
    V: Conversion<V> + Scalar + Copy + SimdComplexField,
    V::SimdRealField: Num + Conversion<<V as SimdComplexField>::SimdRealField> + Scalar + Copy,
{
    type Output = Quantity<D, U, <V as SimdComplexField>::SimdRealField>;
    fn norm(&self) -> Self::Output {
        quantity!(self.value.norm())
    }
}

impl<D: ?Sized, U: ?Sized, V> DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + Copy,
{
    pub fn new(x: Quantity<D, U, V>, y: Quantity<D, U, V>, z: Quantity<D, U, V>) -> Self {
        Self {
            dimension: PhantomData,
            units: PhantomData,
            value: Vector3::new(x.value, y.value, z.value),
        }
    }

    pub fn get_x(&self) -> Quantity<D, U, V> {
        quantity!(self.value.x)
    }

    pub fn get_y(&self) -> Quantity<D, U, V> {
        quantity!(self.value.y)
    }

    pub fn get_z(&self) -> Quantity<D, U, V> {
        quantity!(self.value.z)
    }
}

impl<D: ?Sized> MyVector3<D>
where
    D: Dimension,
{
    pub fn angle(self, other: &Self) -> Angle {
        Angle::new::<radian>(Vector3::from(self).angle(&other.value))
    }

    pub fn pivot_z(self, angle: Angle) -> Self {
        (Rotation3::from_axis_angle(&Vector3::z_axis(), angle.get::<radian>())
            * Vector3::from(self))
        .into()
    }

    pub fn pivot_y(self, angle: Angle) -> Self {
        (Rotation3::from_axis_angle(&Vector3::y_axis(), angle.get::<radian>())
            * Vector3::from(self))
        .into()
    }

    pub fn pivot_x(self, angle: Angle) -> Self {
        (Rotation3::from_axis_angle(&Vector3::x_axis(), angle.get::<radian>())
            * Vector3::from(self))
        .into()
    }
}
