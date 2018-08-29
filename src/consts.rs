pub use self::{
    physics::*,
    conversions::{
        distance::*,
        time::*,
        weightmass::*,
        temperature::*,
        derived::*,
    },
};

pub mod physics {
    pub const GRAVITY: f64 = -9.80665; // Local gravity in m/s
    pub const UNIVERSAL_GAS: f64 = 8.314; // Universal gas constant (J/K*mol)
    pub const MOLAR_DRY: f64 = 0.0289644; // Molar mass of dry air (kg/mol)
    pub const MOLAR_VAPOR: f64 = 0.018016; // Molar mass of water vapor (kg/mol)
}
pub mod conversions {
    pub mod distance {
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

        pub const MILES_TO_METERS: f64 = MILES_TO_FEET * FEET_TO_METERS;
        pub const METERS_TO_MILES: f64 = 1.0 / MILES_TO_METERS;

        pub const YARDS_TO_METERS: f64 = YARDS_TO_FEET * FEET_TO_METERS;
        pub const METERS_TO_YARDS: f64 = 1.0 / YARDS_TO_METERS;

        pub const INCHES_TO_METERS: f64 = INCHES_TO_FEET * FEET_TO_METERS;
        pub const METERS_TO_INCHES: f64 = 1.0 / INCHES_TO_METERS;
    }
    pub mod time {
        pub const HOURS_TO_MINUTES: f64 = 60.0;
        pub const MINUTES_TO_HOURS: f64 = 1.0 / HOURS_TO_MINUTES;

        pub const MINUTES_TO_SECONDS: f64 = 60.0;
        pub const HOURS_TO_SECONDS: f64 = HOURS_TO_MINUTES * MINUTES_TO_SECONDS;

        pub const SECONDS_TO_HOURS: f64 = 1.0 / HOURS_TO_SECONDS;
        pub const SECONDS_TO_MINUTES: f64 = 1.0 / MINUTES_TO_SECONDS;
    }
    pub mod weightmass {
        pub const LBS_TO_GRAINS: f64 = 7000.0;
        pub const GRAINS_TO_LBS: f64 = 1.0 / LBS_TO_GRAINS;

        pub const GRAINS_TO_KG: f64 = GRAINS_TO_LBS * LBS_TO_KG;
        pub const KG_TO_GRAINS: f64 = 1.0 / GRAINS_TO_KG;

        pub const LBS_TO_KG: f64 = 0.45359237;
        pub const KG_TO_LBS: f64 = 1.0 / LBS_TO_KG;
    }
    pub mod temperature {
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
        use super::{distance::*, time::*, weightmass::*};
        pub const LBPF3_TO_KGPM3: f64 = LBS_TO_KG / (FEET_TO_METERS * FEET_TO_METERS * FEET_TO_METERS);
        pub const KGPM3_TO_LBPF3: f64 = 1.0 / LBPF3_TO_KGPM3;

        pub const INHG_TO_PA: f64 = 3386.38;
        pub const PA_TO_INHG: f64 = 1.0 / INHG_TO_PA;

        pub const MPH_TO_MPS: f64 = MPH_TO_FPS * FPS_TO_MPS;
        pub const MPS_TO_MPH: f64 = 1.0 / MPH_TO_MPS;

        pub const MPH_TO_FPS: f64 = MILES_TO_FEET / HOURS_TO_SECONDS;
        pub const FPS_TO_MPH: f64 = 1.0 / MPH_TO_FPS;

        pub const FPS_TO_MPS: f64 = FEET_TO_METERS;
        pub const MPS_TO_FPS: f64 = 1.0 / FPS_TO_MPS;
    }
}
