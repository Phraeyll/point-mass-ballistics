pub use self::{
    add::*, add_assign::*, div::*, div_assign::*, mul::*, mul_assign::*, sub::*, sub_assign::*,
};
use crate::util::{
    marker, radian, Angle, Conversion, Dimension, MyUnits, Numeric, Quantity, Units, ISQ,
};

use core::ops::Add;
use std::{fmt, marker::PhantomData};

use alga::general::{ComplexField, Ring};
use nalgebra::{base::Scalar, Rotation3, Vector3};
use num_traits::Num;
use typenum::operator_aliases::{Diff, Sum};

mod add;
mod add_assign;
mod div;
mod div_assign;
mod mul;
mod mul_assign;
mod sub;
mod sub_assign;

pub type MyVector3<D> = DimVector3<D, MyUnits, Numeric>;
pub struct DimVector3<D: ?Sized, U: ?Sized, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar,
{
    dimension: PhantomData<D>,
    units: PhantomData<U>,
    pub value: Vector3<V>,
}

#[macro_export]
macro_rules! quantity {
    ($value:expr) => {
        Quantity {
            dimension: ::std::marker::PhantomData,
            units: ::std::marker::PhantomData,
            value: $value,
        }
    };
}
#[macro_export]
macro_rules! my_quantity {
    ($value:expr) => {
        MyQuantity {
            dimension: ::std::marker::PhantomData,
            units: ::std::marker::PhantomData,
            value: $value,
        }
    };
}

#[macro_export]
macro_rules! vector3 {
    ($value:expr) => {
        DimVector3 {
            dimension: ::std::marker::PhantomData,
            units: ::std::marker::PhantomData,
            value: $value,
        }
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
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vector3: {{ x: {}, y: {}, z: {} }}",
            self.value.x, self.value.y, self.value.z
        )
    }
}

impl<D: ?Sized, U: ?Sized, V> Copy for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar,
{
}
impl<D: ?Sized, U: ?Sized, V> Clone for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    V: Num + Conversion<V> + Scalar,
{
    fn clone(&self) -> Self {
        *self
    }
}

pub trait Cross<Rhs = Self> {
    type Output;
    fn cross(&self, rhs: Rhs) -> Self::Output;
}

pub trait Vectors<D: ?Sized, U: ?Sized, V> {
    type Quantity;
    type Norm;

    fn new(x: Self::Quantity, y: Self::Quantity, z: Self::Quantity) -> Self;
    fn norm(&self) -> Self::Norm;
    fn get_x(&self) -> Self::Quantity;
    fn get_y(&self) -> Self::Quantity;
    fn get_z(&self) -> Self::Quantity;
}

impl<Dl: ?Sized, Dr: ?Sized, Ul: ?Sized, Ur: ?Sized, V> Cross<&DimVector3<Dr, Ur, V>>
    for DimVector3<Dl, Ul, V>
where
    Dl: Dimension,
    Dr: Dimension,
    Ul: Units<V>,
    Ur: Units<V>,
    V: Num + Conversion<V> + Scalar + Ring,
    Dl::Kind: marker::Mul,
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
        vector3!(self.value.cross(&rhs.value))
    }
}

impl<D: ?Sized, U: ?Sized, V> Vectors<D, U, V> for DimVector3<D, U, V>
where
    D: Dimension,
    U: Units<V>,
    U: Units<<V as ComplexField>::RealField>,
    V: Num + Conversion<V> + Scalar + ComplexField,
    V::RealField: Num + Conversion<<V as ComplexField>::RealField> + Scalar,
{
    type Quantity = Quantity<D, U, V>;
    type Norm = Quantity<D, U, <V as ComplexField>::RealField>;

    fn new(x: Self::Quantity, y: Self::Quantity, z: Self::Quantity) -> Self {
        vector3!(Vector3::new(x.value, y.value, z.value))
    }
    fn norm(&self) -> Self::Norm {
        quantity!(self.value.norm())
    }
    fn get_x(&self) -> Self::Quantity {
        quantity!(self.value.x)
    }
    fn get_y(&self) -> Self::Quantity {
        quantity!(self.value.y)
    }
    fn get_z(&self) -> Self::Quantity {
        quantity!(self.value.z)
    }
}

impl<D: ?Sized> MyVector3<D>
where
    D: Dimension,
{
    pub fn angle(&self, other: &Self) -> Angle {
        Angle::new::<radian>(self.value.angle(&other.value))
    }
    pub fn pivot_z(&self, angle: Angle) -> Self {
        vector3!(Rotation3::from_axis_angle(&Vector3::z_axis(), angle.get::<radian>()) * self.value)
    }
    pub fn pivot_y(&self, angle: Angle) -> Self {
        vector3!(Rotation3::from_axis_angle(&Vector3::y_axis(), angle.get::<radian>()) * self.value)
    }
    pub fn pivot_x(&self, angle: Angle) -> Self {
        vector3!(Rotation3::from_axis_angle(&Vector3::x_axis(), angle.get::<radian>()) * self.value)
    }
}
