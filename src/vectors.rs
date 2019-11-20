pub use self::{add::*, add_assign::*, div::*, mul::*, sub::*, sub_assign::*};
use crate::util::{marker, radian, Angle, Dimension, MyQuantity, Numeric};

use core::ops::Add;
use std::{fmt, marker::PhantomData};

use nalgebra::{Rotation3, Vector3};

mod add;
mod add_assign;
mod div;
mod mul;
mod sub;
mod sub_assign;

pub struct MyVector3<D: ?Sized>
where
    D: Dimension,
{
    dimension: PhantomData<D>,
    pub value: Vector3<Numeric>,
}

#[macro_export]
macro_rules! quantity {
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
        MyVector3 {
            dimension: ::std::marker::PhantomData,
            value: $value,
        }
    };
}

impl<D: ?Sized> fmt::Debug for MyVector3<D>
where
    D: Dimension,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vector3: {{ x: {}, y: {}, z: {} }}",
            self.value.x, self.value.y, self.value.z
        )
    }
}

impl<D: ?Sized> Copy for MyVector3<D> where D: Dimension {}
impl<D: ?Sized> Clone for MyVector3<D>
where
    D: Dimension,
{
    fn clone(&self) -> MyVector3<D> {
        *self
    }
}

pub trait Cross<Rhs = Self> {
    type Output;
    fn cross(&self, rhs: Rhs) -> Self::Output;
}

pub trait Vectors<V = Self> {
    type Quantity;

    fn new(x: Self::Quantity, y: Self::Quantity, z: Self::Quantity) -> Self;
    fn norm(&self) -> Self::Quantity;
    fn angle(&self, other: &Self) -> Angle;
    fn get_x(&self) -> Self::Quantity;
    fn get_y(&self) -> Self::Quantity;
    fn get_z(&self) -> Self::Quantity;

    fn pivot_z(&self, angle: Angle) -> Self;
    fn pivot_y(&self, angle: Angle) -> Self;
    fn pivot_x(&self, angle: Angle) -> Self;
}

impl<Dl: ?Sized, Dr: ?Sized> Cross<&MyVector3<Dr>> for MyVector3<Dl>
where
    Dl: Dimension,
    Dr: Dimension,
    Dl::Kind: marker::Mul,
    Dl::L: Add<Dr::L>,
    Dl::M: Add<Dr::M>,
    Dl::T: Add<Dr::T>,
    Dl::I: Add<Dr::I>,
    Dl::Th: Add<Dr::Th>,
    Dl::N: Add<Dr::N>,
    Dl::J: Add<Dr::J>,
{
    type Output = MyVector3<SumDimension<Dl, Dr>>;
    fn cross(&self, rhs: &MyVector3<Dr>) -> Self::Output {
        vector3!(self.value.cross(&rhs.value))
    }
}

impl<D: ?Sized> Vectors for MyVector3<D>
where
    D: Dimension,
{
    type Quantity = MyQuantity<D>;

    fn new(x: Self::Quantity, y: Self::Quantity, z: Self::Quantity) -> Self {
        vector3!(Vector3::new(x.value, y.value, z.value))
    }
    fn norm(&self) -> Self::Quantity {
        quantity!(self.value.norm())
    }
    fn angle(&self, other: &Self) -> Angle {
        Angle::new::<radian>(self.value.angle(&other.value))
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
    fn pivot_z(&self, angle: Angle) -> Self {
        vector3!(Rotation3::from_axis_angle(&Vector3::z_axis(), angle.get::<radian>()) * self.value)
    }
    fn pivot_y(&self, angle: Angle) -> Self {
        vector3!(Rotation3::from_axis_angle(&Vector3::y_axis(), angle.get::<radian>()) * self.value)
    }
    fn pivot_x(&self, angle: Angle) -> Self {
        vector3!(Rotation3::from_axis_angle(&Vector3::x_axis(), angle.get::<radian>()) * self.value)
    }
}
