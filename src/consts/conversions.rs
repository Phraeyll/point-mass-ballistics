pub mod multiplicative {
    pub mod similar {
        // Basis
            // Length
        pub const MILES_TO_YARDS: f64 = 1760.0;
        pub const YARDS_TO_MILES: f64 = 1.0 / MILES_TO_YARDS;

        pub const YARDS_TO_FEET: f64 = 3.0;
        pub const FEET_TO_YARDS: f64 = 1.0 / YARDS_TO_FEET;

        pub const FEET_TO_INCHES: f64 = 12.0;
        pub const INCHES_TO_FEET: f64 = 1.0 / FEET_TO_INCHES;

        pub const INCHES_TO_THOU: f64 = 1000.0;
        pub const THOU_TO_INCHES: f64 = 1.0 / INCHES_TO_THOU;
                // Shortcuts
        pub const MILES_TO_FEET: f64 = MILES_TO_YARDS * YARDS_TO_FEET;
        pub const FEET_TO_MILES: f64 = 1.0 / MILES_TO_FEET;

        pub const MILES_TO_INCHES: f64 = MILES_TO_FEET * FEET_TO_INCHES;
        pub const INCHES_TO_MILES: f64 = 1.0 / MILES_TO_INCHES;
            // Time
        pub const HOURS_TO_MINUTES: f64 = 60.0;
        pub const MINUTES_TO_HOURS: f64 = 1.0 / HOURS_TO_MINUTES;

        pub const MINUTES_TO_SECONDS: f64 = 60.0;
        pub const SECONDS_TO_MINUTES: f64 = 1.0 / MINUTES_TO_SECONDS;
                // Shortcuts
        pub const HOURS_TO_SECONDS: f64 = HOURS_TO_MINUTES * MINUTES_TO_SECONDS;
        pub const SECONDS_TO_HOURS: f64 = 1.0 / HOURS_TO_SECONDS;
            // Mass/Weight
        pub const LBS_TO_GRAINS: f64 = 7000.0;
        pub const GRAINS_TO_LBS: f64 = 1.0 / LBS_TO_GRAINS;
        // Derived
            // Position/Length Derivatives (Velocity, Acceleration, etc.)
        pub const MILES_PER_HOUR_TO_FEET_PER_SECOND: f64 = MILES_TO_FEET / HOURS_TO_SECONDS;
        pub const FEET_PER_SECOND_TO_MILES_PER_HOUR: f64 = 1.0 / MILES_PER_HOUR_TO_FEET_PER_SECOND;
    }

    pub mod different {
        // Basis
            // Length
        pub const FEET_TO_METERS: f64 = 0.3048;
        pub const METERS_TO_FEET: f64 = 1.0 / FEET_TO_METERS;
            // Mass/Weight
        pub const LBS_TO_KG: f64 = 0.45359237;
        pub const KG_TO_LBS: f64 = 1.0 / LBS_TO_KG;
            // Force (Pressure)
        pub const INHG_TO_PA: f64 = 3386.38;
        pub const PA_TO_INHG: f64 = 1.0 / INHG_TO_PA;
            // Temperature
        pub const F_TO_CK: f64 = 5.0 / 9.0;
        pub const CK_TO_F: f64 = 1.0 / F_TO_CK;
        // Scaled
            // Length
        pub const INCHES_TO_METERS: f64 = super::similar::INCHES_TO_FEET * FEET_TO_METERS;
        pub const METERS_TO_INCHES: f64 = 1.0 / INCHES_TO_METERS;

        pub const YARDS_TO_METERS: f64 = super::similar::YARDS_TO_FEET * FEET_TO_METERS;
        pub const METERS_TO_YARDS: f64 = 1.0 / YARDS_TO_METERS;

        pub const MILES_TO_METERS: f64 = super::similar::MILES_TO_YARDS * YARDS_TO_METERS;
        pub const METERS_TO_MILES: f64 = 1.0 / MILES_TO_METERS;
            // Mass/Weight
        pub const GRAINS_TO_KG: f64 = super::similar::GRAINS_TO_LBS * LBS_TO_KG;
        pub const KG_TO_GRAINS: f64 = 1.0 / GRAINS_TO_KG;
        // Derived
            // Position/Length Derivatives (Velocity, Acceleration, etc.)
                // Shortcuts
        pub const MILES_PER_HOUR_TO_METERS_PER_SECOND: f64 = super::similar::MILES_PER_HOUR_TO_FEET_PER_SECOND * FEET_TO_METERS;
        pub const METERS_PER_SECOND_TO_MILES_PER_HOUR: f64 = 1.0 / MILES_PER_HOUR_TO_METERS_PER_SECOND;
    }
}
pub mod additive {
    pub mod different {
        pub const C_TO_K: f64 = 273.15;
        pub const K_TO_C: f64 = -C_TO_K;

        pub const F_TO_K: f64 = 459.67;
        pub const K_TO_F: f64 = -F_TO_K;

        pub const C_TO_F: f64 = 32.0;
        pub const F_TO_C: f64 = -C_TO_F;
    }
}
