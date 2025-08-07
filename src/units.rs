use crate::Numeric;

pub use uom::si::{
    acceleration, angle, angular_velocity, area, energy,
    f64::{
        Acceleration, Angle, AngularVelocity, Area, ArealMassDensity, Energy, Length, Mass,
        MassDensity, MolarHeatCapacity, MolarMass, Pressure, Ratio, ReciprocalLength,
        ThermodynamicTemperature, Time, Velocity,
    },
    force, length, mass, mass_density, molar_mass, pressure, reciprocal_length,
    thermodynamic_temperature, time, velocity,
};

pub(crate) use uom::{
    ConstZero, Conversion,
    num_traits::Num,
    si::{Dimension, ISQ, Quantity, SI, Units},
    typenum,
};

pub(crate) type MyUnits = SI<Numeric>;
pub(crate) type MyQuantity<D> = Quantity<D, MyUnits, Numeric>;

macro_rules! my_quantity {
    ($value:expr) => {
        $crate::units::MyQuantity {
            dimension: ::std::marker::PhantomData,
            units: ::std::marker::PhantomData,
            value: $value,
        }
    };
}
pub(crate) use my_quantity;

macro_rules! quantity {
    ($value:expr) => {
        $crate::units::Quantity {
            dimension: ::std::marker::PhantomData,
            units: ::std::marker::PhantomData,
            value: $value,
        }
    };
}
pub(crate) use quantity;
