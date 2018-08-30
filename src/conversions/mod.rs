pub use self::{derived::*, length::*, temperature::*, time::*, weight_mass::*};

mod consts;

use self::consts::*;

mod length {
    use super::*;
    use self::Length::*;
    pub enum Length {
        Meters(f64),
        Miles(f64),
        Yards(f64),
        Feet(f64),
        Inches(f64),
    }
    impl From<Length> for f64 {
        fn from(u: Length) -> f64 {
            match u {
                Meters(u) => u,
                Miles(u) => u,
                Yards(u) => u,
                Feet(u) => u,
                Inches(u) => u,
            }
        }
    }
    impl self::Length {
        pub fn to_meters(self) -> Self {
            match self {
                u @ Meters(_) => u,
                Miles(u) => Meters(u * MILES_TO_METERS),
                Yards(u) => Meters(u * YARDS_TO_METERS),
                Feet(u) => Meters(u * FEET_TO_METERS),
                Inches(u) => Meters(u * INCHES_TO_METERS),
            }
        }
        pub fn to_inches(self) -> Self {
            match self {
                u @ Inches(_) => u,
                Meters(u) => Inches(u * METERS_TO_INCHES),
                Yards(u) => Inches(u * YARDS_TO_INCHES),
                Feet(u) => Inches(u * FEET_TO_INCHES),
                Miles(u) => Inches(u * MILES_TO_INCHES),
            }
        }
        pub fn to_yards(self) -> Self {
            match self {
                u @ Yards(_) => u,
                Meters(u) => Yards(u * METERS_TO_YARDS),
                Feet(u) => Yards(u * FEET_TO_YARDS),
                Miles(u) => Yards(u * MILES_TO_YARDS),
                Inches(u) => Yards(u * INCHES_TO_YARDS),
            }
        }
        pub fn to_miles(self) -> Self {
            match self {
                u @ Miles(_) => u,
                Meters(u) => Miles(u * METERS_TO_MILES),
                Feet(u) => Miles(u * FEET_TO_MILES),
                Yards(u) => Miles(u * YARDS_TO_MILES),
                Inches(u) => Miles(u * INCHES_TO_MILES),
            }
        }
        pub fn to_feet(self) -> Self {
            match self {
                u @ Feet(_) => u,
                Meters(u) => Feet(u * METERS_TO_FEET),
                Yards(u) => Feet(u * YARDS_TO_FEET),
                Miles(u) => Feet(u * MILES_TO_FEET),
                Inches(u) => Feet(u * INCHES_TO_FEET),
            }
        }
    }
}
mod time {
    use super::*;
    use self::Time::*;
    pub enum Time {
        Hours(f64),
        Minutes(f64),
        Seconds(f64),
    }
    impl From<Time> for f64 {
        fn from(u: Time) -> f64 {
            match u {
                Hours(u) => u,
                Minutes(u) => u,
                Seconds(u) => u,
            }
        }
    }
    impl self::Time {
        pub fn to_hours(self) -> Self {
            match self {
                u @ Hours(_) => u,
                Minutes(u) => Hours(u * MINUTES_TO_HOURS),
                Seconds(u) => Hours(u * SECONDS_TO_HOURS),
            }
        }
        pub fn to_minutes(self) -> Self {
            match self {
                u @ Minutes(_) => u,
                Hours(u) => Minutes(u * HOURS_TO_MINUTES),
                Seconds(u) => Minutes(u * SECONDS_TO_MINUTES),
            }
        }
        pub fn to_seconds(self) -> Self {
            match self {
                u @ Seconds(_) => u,
                Hours(u) => Seconds(u * HOURS_TO_SECONDS),
                Minutes(u) => Seconds(u * MINUTES_TO_SECONDS),
            }
        }
    }
}
mod weight_mass {
    use super::*;
    use self::WeightMass::*;
    pub enum WeightMass {
        Grains(f64),
        Lbs(f64),
        Kgs(f64),
    }
    impl From<WeightMass> for f64 {
        fn from(u: WeightMass) -> f64 {
            match u {
                Grains(u) => u,
                Lbs(u) => u,
                Kgs(u) => u,
            }
        }
    }
    impl self::WeightMass {
        pub fn to_grains(self) -> Self {
            match self {
                u @ Grains(_) => u,
                Lbs(u) => Grains(u * LBS_TO_GRAINS),
                Kgs(u) => Grains(u * KGS_TO_GRAINS),
            }
        }
        pub fn to_lbs(self) -> Self {
            match self {
                u @ Lbs(_) => u,
                Grains(u) => Lbs(u * GRAINS_TO_LBS),
                Kgs(u) => Lbs(u * KGS_TO_LBS),
            }
        }
        pub fn to_kgs(self) -> Self {
            match self {
                u @ Kgs(_) => u,
                Grains(u) => Kgs(u * GRAINS_TO_KGS),
                Lbs(u) => Kgs(u * LBS_TO_KGS),
            }
        }
    }
}
mod temperature {
    use super::*;
    use self::Temperature::*;
    pub enum Temperature {
        C(f64),
        K(f64),
        F(f64),
    }
    impl From<Temperature> for f64 {
        fn from(u: Temperature) -> f64 {
            match u {
                C(u) => u,
                K(u) => u,
                F(u) => u,
            }
        }
    }
    impl self::Temperature {
        pub fn to_celsius(self) -> Self {
            match self {
                u @ C(_) => u,
                K(u) => C(u + K_TO_C),
                F(u) => C((u + F_TO_C) * F_TO_CK),
            }
        }
        pub fn to_kelvin(self) -> Self {
            match self {
                u @ K(_) => u,
                C(u) => K(u + C_TO_K),
                F(u) => K((u + F_TO_K) * F_TO_CK),
            }
        }
        pub fn to_fahrenheit(self) -> Self {
            match self {
                u @ F(_) => u,
                C(u) => F((u * CK_TO_F) + C_TO_F),
                K(u) => F((u * CK_TO_F) + K_TO_F),
            }
        }
    }
}
mod derived {
    use super::*;
    use self::{Density::*, Pressure::*, Velocity::*};

    pub enum Pressure {
        Pascals(f64),
        Inhg(f64),
    }
    impl From<Pressure> for f64 {
        fn from(u: Pressure) -> f64 {
            match u {
                Pascals(u) => u,
                Inhg(u) => u,
            }
        }
    }
    impl Pressure {
        pub fn to_pascals(self) -> Self {
            match self {
                u @ Pascals(_) => u,
                Inhg(u) => Pascals(u * INHG_TO_PASCALS),
            }
        }
        pub fn to_inhg(self) -> Self {
            match self {
                u @ Inhg(_) => u,
                Pascals(u) => Inhg(u * PASCALS_TO_INHG),
            }
        }
    }

    pub enum Density {
        Kgpm3(f64),
        Lbpf3(f64),
    }
    impl From<Density> for f64 {
        fn from(u: Density) -> f64 {
            match u {
                Kgpm3(u) => u,
                Lbpf3(u) => u,
            }
        }
    }
    impl self::Density {
        pub fn to_kgpm3(self) -> Self {
            match self {
                u @ Kgpm3(_) => u,
                Lbpf3(u) => Kgpm3(u * LBPF3_TO_KGPM3),
            }
        }
        pub fn to_lbpf3(self) -> Self {
            match self {
                u @ Lbpf3(_) => u,
                Kgpm3(u) => Lbpf3(u * KGPM3_TO_LBPF3),
            }
        }
    }

    pub enum Velocity {
        Mps(f64),
        Mph(f64),
        Fps(f64),
    }
    impl From<Velocity> for f64 {
        fn from(u: Velocity) -> f64 {
            match u {
                Mps(u) => u,
                Mph(u) => u,
                Fps(u) => u,
            }
        }
    }
    impl self::Velocity {
        pub fn to_mps(self) -> Self {
            match self {
                u @ Mps(_) => u,
                Mph(u) => Mps(u * MPH_TO_MPS),
                Fps(u) => Mps(u * FPS_TO_MPS),
            }
        }
        pub fn to_mph(self) -> Self {
            match self {
                u @ Mph(_) => u,
                Mps(u) => Mph(u * MPS_TO_MPH),
                Fps(u) => Mph(u * FPS_TO_MPH),
            }
        }
        pub fn to_fps(self) -> Self {
            match self {
                u @ Fps(_) => u,
                Mps(u) => Fps(u * MPS_TO_FPS),
                Mph(u) => Fps(u * MPH_TO_FPS),
            }
        }
    }
}
