use crate::Numeric;

pub use uom::{
    fmt::DisplayStyle,
    si::{
        acceleration::{self, foot_per_second_squared, meter_per_second_squared},
        amount_of_substance::{self, mole},
        angle::{self, degree, minute as moa, radian},
        angular_velocity::{self, radian_per_second},
        area::{self, square_inch, square_meter},
        electric_current::{self, ampere},
        energy::{self, foot_pound, joule},
        f64::*,
        fmt::{Arguments, QuantityArguments},
        force::{self},
        length::{self, inch, meter, yard},
        luminous_intensity::{self, candela},
        mass::{self, grain, kilogram, pound},
        mass_density::{self, kilogram_per_cubic_meter},
        molar_mass::{self},
        pressure::{self, inch_of_mercury, pascal},
        ratio::{self},
        thermodynamic_temperature::{
            self as temperature, degree_celsius as celsius, degree_fahrenheit as fahrenheit, kelvin,
        },
        time::{self, second},
        velocity::{self, foot_per_second, meter_per_second, mile_per_hour},
    },
    str::ParseQuantityError,
};

pub(crate) use uom::{
    si::{Dimension, Quantity, Units, ISQ, SI},
    typenum, Conversion,
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
