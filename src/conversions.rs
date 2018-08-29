pub use self::{
    derived::*, length::*, temperature::*, time::*, weight_mass::*,
};

pub mod length {
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
                Length::Meters(u) => u,
                Length::Miles(u) => u,
                Length::Yards(u) => u,
                Length::Feet(u) => u,
                Length::Inches(u) => u,
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
        pub fn to_yards(self) -> Self{
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
    pub const MILES_TO_YARDS: f64 = 1760.0;
    pub const YARDS_TO_MILES: f64 = 1.0 / MILES_TO_YARDS;

    pub const YARDS_TO_FEET: f64 = 3.0;
    pub const FEET_TO_YARDS: f64 = 1.0 / YARDS_TO_FEET;

    pub const FEET_TO_INCHES: f64 = 12.0;
    pub const INCHES_TO_FEET: f64 = 1.0 / FEET_TO_INCHES;

    pub const FEET_TO_METERS: f64 = 0.3048;
    pub const METERS_TO_FEET: f64 = 1.0 / FEET_TO_METERS;

    pub const MILES_TO_FEET: f64 = MILES_TO_YARDS * YARDS_TO_FEET;
    pub const FEET_TO_MILES: f64 = 1.0 / MILES_TO_FEET;

    pub const YARDS_TO_INCHES: f64 = YARDS_TO_FEET * FEET_TO_INCHES;
    pub const INCHES_TO_YARDS: f64 = 1.0 / YARDS_TO_INCHES;

    pub const MILES_TO_INCHES: f64 = MILES_TO_YARDS * YARDS_TO_INCHES;
    pub const INCHES_TO_MILES: f64 = 1.0 / MILES_TO_INCHES;

    pub const MILES_TO_METERS: f64 = MILES_TO_FEET * FEET_TO_METERS;
    pub const METERS_TO_MILES: f64 = 1.0 / MILES_TO_METERS;

    pub const YARDS_TO_METERS: f64 = YARDS_TO_FEET * FEET_TO_METERS;
    pub const METERS_TO_YARDS: f64 = 1.0 / YARDS_TO_METERS;

    pub const INCHES_TO_METERS: f64 = INCHES_TO_FEET * FEET_TO_METERS;
    pub const METERS_TO_INCHES: f64 = 1.0 / INCHES_TO_METERS;
}
pub mod time {
    use self::Time::*;
    pub enum Time {
        Hours(f64),
        Minutes(f64),
        Seconds(f64),
    }
    impl From<Time> for f64 {
        fn from(u: Time) -> f64 {
            match u {
                Time::Hours(u) => u,
                Time::Minutes(u) => u,
                Time::Seconds(u) => u,
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
        pub fn to_seconds(self) -> Self{
            match self {
                u @ Seconds(_) => u,
                Hours(u) => Seconds(u * HOURS_TO_SECONDS),
                Minutes(u) => Seconds(u * MINUTES_TO_SECONDS),
            }
        }
    }
    pub const HOURS_TO_MINUTES: f64 = 60.0;
    pub const MINUTES_TO_HOURS: f64 = 1.0 / HOURS_TO_MINUTES;

    pub const MINUTES_TO_SECONDS: f64 = 60.0;
    pub const HOURS_TO_SECONDS: f64 = HOURS_TO_MINUTES * MINUTES_TO_SECONDS;

    pub const SECONDS_TO_HOURS: f64 = 1.0 / HOURS_TO_SECONDS;
    pub const SECONDS_TO_MINUTES: f64 = 1.0 / MINUTES_TO_SECONDS;
}
pub mod weight_mass {
    use self::WeightMass::*;
    pub enum WeightMass {
        Grains(f64),
        Lbs(f64),
        Kgs(f64),
    }
    impl From<WeightMass> for f64 {
        fn from(u: WeightMass) -> f64 {
            match u {
                WeightMass::Grains(u) => u,
                WeightMass::Lbs(u) => u,
                WeightMass::Kgs(u) => u,
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
        pub fn to_kgs(self) -> Self{
            match self {
                u @ Kgs(_) => u,
                Grains(u) => Kgs(u * GRAINS_TO_KGS),
                Lbs(u) => Kgs(u * LBS_TO_KGS),
            }
        }
    }
    pub const LBS_TO_GRAINS: f64 = 7000.0;
    pub const GRAINS_TO_LBS: f64 = 1.0 / LBS_TO_GRAINS;

    pub const GRAINS_TO_KGS: f64 = GRAINS_TO_LBS * LBS_TO_KGS;
    pub const KGS_TO_GRAINS: f64 = 1.0 / GRAINS_TO_KGS;

    pub const LBS_TO_KGS: f64 = 0.45359237;
    pub const KGS_TO_LBS: f64 = 1.0 / LBS_TO_KGS;
}
pub mod temperature {
    use self::Temperature::*;
    pub enum Temperature {
        C(f64),
        K(f64),
        F(f64),
    }
    impl From<Temperature> for f64 {
        fn from(u: Temperature) -> f64 {
            match u {
                Temperature::C(u) => u,
                Temperature::K(u) => u,
                Temperature::F(u) => u,
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
    pub const F_TO_CK: f64 = 5.0 / 9.0;
    pub const CK_TO_F: f64 = 1.0 / F_TO_CK;

    // Additive
    pub const C_TO_K: f64 = 273.15;
    pub const K_TO_C: f64 = -C_TO_K;

    // Additive
    pub const F_TO_K: f64 = 459.67;
    pub const K_TO_F: f64 = -F_TO_K;

    // Additive
    pub const F_TO_C: f64 = -32.0;
    pub const C_TO_F: f64 = -F_TO_C;

}
pub mod derived {
    use super::{length::*, time::*, weight_mass::*};
    use self::{Pressure::*, Density::*, Velocity::*};

    pub enum Pressure {
        Pascals(f64),
        Inhg(f64),
    }
    impl From<Pressure> for f64 {
        fn from(u: Pressure) -> f64 {
            match u {
                Pressure::Pascals(u) => u,
                Pressure::Inhg(u) => u,
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

    pub const INHG_TO_PASCALS: f64 = 3386.38;
    pub const PASCALS_TO_INHG: f64 = 1.0 / INHG_TO_PASCALS;

    pub enum Density {
        Kgpm3(f64),
        Lbpf3(f64),
    }
    impl From<Density> for f64 {
        fn from(u: Density) -> f64 {
            match u {
                Density::Kgpm3(u) => u,
                Density::Lbpf3(u) => u,
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
    pub const LBPF3_TO_KGPM3: f64 =
        LBS_TO_KGS / (FEET_TO_METERS * FEET_TO_METERS * FEET_TO_METERS);
    pub const KGPM3_TO_LBPF3: f64 = 1.0 / LBPF3_TO_KGPM3;

    pub enum Velocity {
        Mps(f64),
        Mph(f64),
        Fps(f64),
    }
    impl From<Velocity> for f64 {
        fn from(u: Velocity) -> f64 {
            match u {
                Velocity::Mps(u) => u,
                Velocity::Mph(u) => u,
                Velocity::Fps(u) => u,
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
    pub const MPH_TO_MPS: f64 = MPH_TO_FPS * FPS_TO_MPS;
    pub const MPS_TO_MPH: f64 = 1.0 / MPH_TO_MPS;

    pub const MPH_TO_FPS: f64 = MILES_TO_FEET / HOURS_TO_SECONDS;
    pub const FPS_TO_MPH: f64 = 1.0 / MPH_TO_FPS;

    pub const FPS_TO_MPS: f64 = FEET_TO_METERS;
    pub const MPS_TO_FPS: f64 = 1.0 / FPS_TO_MPS;
}
