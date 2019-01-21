use nalgebra::Vector3;

use super::*;

use std::ops::Mul;

const GRAVITY: Numeric = -9.806_65; // Local gravity in m/s
const UNIVERSAL_GAS: Numeric = 8.314_459_8; // Universal gas constant (J/K*mol)
const MOLAR_DRY: Numeric = 0.028_964_4; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: Numeric = 0.018_016; // Molar mass of water vapor (kg/mol)
const ADIABATIC_INDEX_AIR: Numeric = 1.4; // Adiabatic index of air, mostly diatomic gas
const ANGULAR_VELOCITY_EARTH: Numeric = 0.000_072_921_159; // Angular velocity of earth, (radians)

pub struct Simulation<'p> {
    pub(crate) projectile: &'p Projectile,
    pub(crate) scope: &'p Scope,
    pub(crate) conditions: &'p Conditions,
    pub(crate) time_step: Time,
    pub(crate) muzzle_pitch: Angle,
    pub(crate) muzzle_yaw: Angle,
}
impl<'p> Simulation<'p> {
    pub(crate) fn new(
        projectile: &'p Projectile,
        scope: &'p Scope,
        conditions: &'p Conditions,
        time_step: Time,
        muzzle_pitch: Angle,
        muzzle_yaw: Angle,
    ) -> Self {
        Self {
            projectile,
            scope,
            conditions,
            time_step,
            muzzle_pitch,
            muzzle_yaw,
        }
    }
    // Produce a drop table using specified range and step size
    pub fn table(
        &self,
        step: Natural,
        range_start: Natural,
        range_end: Natural,
    ) -> FloatMap<Packet<'_>> {
        let mut iter = self.into_iter().fuse();
        (range_start..=range_end)
            .step_by(step as usize)
            .filter_map(|current_step| {
                iter.by_ref()
                    .find(|p| p.distance() >= Numeric::from(current_step))
                    .map(|p| (p.distance(), p))
            })
            .collect::<FloatMap<_>>()
    }
    // Rotated velocity vector, accounts for muzzle/shooter pitch, and yaw (bearing)
    // Start with velocity value along X unit vector
    pub(crate) fn absolute_projectile_velocity(&self) -> Vector3<Numeric> {
        self.projectile
            .velocity(self.muzzle_pitch, self.muzzle_yaw)
            .pivot_z(self.conditions.other.line_of_sight)
            .pivot_y(self.conditions.other.corrected_azimuth())
    }
    // Velocity vector of wind, only horizontal at the moment
    // Does not adjust according to line of sight, since most would measure wind
    // along relative bearing - I don't think many would factor in a 'downhill' wind for example
    // This would be interresting to think of, however.
    pub(crate) fn absolute_wind_velocity(&self) -> Vector3<Numeric> {
        self.conditions
            .wind
            .velocity()
            .pivot_y(self.conditions.other.corrected_azimuth())
    }
}

pub struct Projectile {
    pub(crate) weight: WeightMass,       // Weight (grains)
    pub(crate) caliber: Length,          // Caliber (inches)
    pub(crate) bc: BallisticCoefficient, // Ballistic Coefficient
    pub(crate) velocity: Velocity,       // Initial velocity (ft/s)
}
impl Projectile {
    // Radius of projectile cross section in meters
    pub(crate) fn radius(&self) -> Numeric {
        self.caliber.to_meters().to_num() / 2.0
    }
    // Area of projectile in meters, used during drag force calculation
    pub(crate) fn area(&self) -> Numeric {
        PI * self.radius().powf(2.0)
    }
    // Mass of projectile in kgs, used during acceleration calculation in simulation iteration
    pub(crate) fn mass(&self) -> Numeric {
        self.weight.to_kgs().into()
    }
    // Sectional density of projectile, defined terms of lbs and inches, yet dimensionless
    pub(crate) fn sd(&self) -> Numeric {
        self.weight.to_lbs().to_num() / self.caliber.to_inches().to_num().powf(2.0)
    }
    // Form factor of projectile, calculated fro Ballistic Coefficient and Sectional Density (sd)
    pub(crate) fn i(&self) -> Numeric {
        self.sd() / self.bc.value()
    }
    pub(crate) fn velocity(&self, muzzle_pitch: Angle, muzzle_yaw: Angle) -> Vector3<Numeric> {
        self.velocity
            .to_mps()
            .to_num()
            .mul(Vector3::x())
            .pivot_z(muzzle_pitch)
            .pivot_y(muzzle_yaw)
    }
}

pub struct Scope {
    pub(crate) height: Length, // Scope Height (inches)
}
impl Scope {
    pub(crate) fn height(&self) -> Vector3<Numeric> {
        self.height.to_meters().to_num().mul(Vector3::y())
    }
}

pub struct Wind {
    pub(crate) velocity: Velocity, // Wind Velocity (miles/hour)
    pub(crate) yaw: Angle,         // Wind Angle (degrees)
}
impl Wind {
    // This vector indicates direction of wind flow, not source of wind
    // so rotate by PI (adding or subtraction should have the same affect)
    // Negative indicates 90 degree wind is from east=>west
    // 0 degree wind is from north=>south (conventional)
    //        (0)
    //         ^
    //         |
    // (+90) <---> (-90)
    //         |
    //         v
    //       (180)
    //
    //  {after rotation(+ PI)}
    //
    //       (180)
    //         ^
    //         |
    // (-90) <---> (+90)
    //         |
    //         v
    //        (0)
    //
    //  {after negation(-)}
    //
    //       (180)
    //         ^
    //         |
    // (+90) <---> (-90)
    //         |
    //         v
    //        (0)
    pub(crate) fn corrected_yaw(&self) -> Angle {
        Angle::Radians(-self.yaw.to_radians().to_num() + PI)
    }
    pub(crate) fn velocity(&self) -> Vector3<Numeric> {
        self.velocity
            .to_mps()
            .to_num()
            .mul(Vector3::x())
            .pivot_y(self.corrected_yaw())
    }
}

// Environmental Conditions and other varialbe for simulation
pub struct Atmosphere {
    pub(crate) temperature: Temperature, // Temperature (F)
    pub(crate) pressure: Pressure,       // Pressure (InHg)
    pub(crate) humidity: Numeric,        // Humidity (0-1)
}
impl Atmosphere {
    // Density of air, using pressure, humidity, and temperature
    pub(crate) fn rho(&self) -> Numeric {
        ((self.pd() * MOLAR_DRY) + (self.pv() * MOLAR_VAPOR)) / (UNIVERSAL_GAS * self.kelvin())
    }
    // Speed of sound at given air density and pressure
    pub(crate) fn speed_of_sound(&self) -> Numeric {
        (ADIABATIC_INDEX_AIR * (self.pa() / self.rho())).sqrt()
    }
    // Pressure of water vapor, Arden Buck equation
    pub(crate) fn pv(&self) -> Numeric {
        self.humidity
            * 611.21
            * ((18.678 - (self.celsius() / 234.5)) * (self.celsius() / (257.14 + self.celsius())))
                .exp()
    }
    // Pressure of dry air
    pub(crate) fn pd(&self) -> Numeric {
        self.pa() - self.pv()
    }
    // Total air pressure in pascals
    pub(crate) fn pa(&self) -> Numeric {
        self.pressure.to_pascals().to_num()
    }
    // Temperature in celsius
    pub(crate) fn celsius(&self) -> Numeric {
        self.temperature.to_celsius().to_num()
    }
    // Temperature in kelvin
    pub(crate) fn kelvin(&self) -> Numeric {
        self.temperature.to_kelvin().to_num()
    }
}

pub struct Other {
    pub(crate) line_of_sight: Angle,  // Line of Sight angle (degrees)
    pub(crate) azimuth: Angle,        // Bearing (0 North, 90 East) (degrees) (Coriolis/Eotvos Effect)
    pub(crate) lattitude: Angle,      // Lattitude (Coriolis/Eotvos Effect)
    pub(crate) gravity: Acceleration, // Gravity (m/s^2)
}
impl Other {
    pub(crate) fn gravity(&self) -> Vector3<Numeric> {
        self.gravity.to_mps2().to_num().mul(Vector3::y())
    }
    // Flip, since circle functions rotate counter-clockwise,
    // 90 degrees is east by compass bearing, but west(left) in trig
    //        (0)
    //         ^
    //         |
    // (+90) <---> (-90)
    //         |
    //         v
    //       (180)
    //
    //  {after negation(-)}
    //
    //        (0)
    //         ^
    //         |
    // (-90) <---> (+90)
    //         |
    //         v
    //       (180)
    pub(crate) fn corrected_azimuth(&self) -> Angle {
        Angle::Radians(-self.azimuth.to_radians().to_num())
    }
    // Angular velocity vector of earth, at current lattitude
    // Can be thought of as vector from center of earth, pointing
    // to lines of lattitude.  Maximum effect at +/-90 degrees (poles)
    pub(crate) fn omega(&self) -> Vector3<Numeric> {
        ANGULAR_VELOCITY_EARTH
            .mul(Vector3::x())
            .pivot_z(self.lattitude)
    }
}

pub struct Conditions {
    pub(crate) wind: Wind,
    pub(crate) atmosphere: Atmosphere,
    pub(crate) other: Other,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            weight: WeightMass::Grains(140.0),
            caliber: Length::Inches(0.264),
            bc: BallisticCoefficient::new(0.305, G7).expect("how"),
            velocity: Velocity::Fps(2710.0),
        }
    }
}
impl Default for Scope {
    fn default() -> Self {
        Self {
            height: Length::Inches(1.5),
        }
    }
}
impl Default for Wind {
    fn default() -> Self {
        Self {
            velocity: Velocity::Mph(0.0),
            yaw: Angle::Radians(0.0),
        }
    }
}
impl Default for Atmosphere {
    fn default() -> Self {
        Self {
            temperature: Temperature::F(68.0),
            pressure: Pressure::Inhg(29.92),
            humidity: 0.0,
        }
    }
}
impl Default for Other {
    fn default() -> Self {
        Self {
            line_of_sight: Angle::Radians(0.0),
            azimuth: Angle::Radians(0.0),
            lattitude: Angle::Radians(0.0),
            gravity: Acceleration::Mps2(GRAVITY),
        }
    }
}
impl Default for Conditions {
    fn default() -> Self {
        Self {
            wind: Wind::default(),
            atmosphere: Atmosphere::default(),
            other: Other::default(),
        }
    }
}
