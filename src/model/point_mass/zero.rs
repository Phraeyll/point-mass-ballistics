use super::*;

use std::error::Error as StdError;
use std::fmt;
use std::fmt::Display as StdDisplay;
use std::result;
use std::str;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error(Box::new(kind))
    }
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    AngleRange(u64, Numeric),
    TerminalVelocity(u64, Numeric),
    AngleNotChanging(u64, Numeric),
}

impl StdDisplay for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::AngleRange(count, angle) => {
                write!(formatter, "{}: Outside Valid Range Error: {}", count, angle)
            }
            ErrorKind::TerminalVelocity(count, angle) => {
                write!(formatter, "{}: Terminal Velocity Error: {}", count, angle)
            }
            ErrorKind::AngleNotChanging(count, angle) => {
                write!(formatter, "{}: Angle Not Changing Error: {}", count, angle)
            }
        }
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        match *self.0 {
            ErrorKind::AngleRange(..) => "Angle out of range",
            ErrorKind::TerminalVelocity(..) => "Terminal velocity reached",
            ErrorKind::AngleNotChanging(..) => "Angle not changing curing iteration",
        }
    }
}

// This angle will trace the longest possible trajectory for a projectile (45 degrees)
const DEG_45: Numeric = FRAC_PI_4;
// Should never try to yaw more than 90 degrees, probably not a necessary check
// Also should never try to pitch this low - not sure if this ever happens in practice
const DEG_90: Numeric = FRAC_PI_2;

struct IterFindAdjustments<'s> {
    sim: &'s mut super::Simulation<'s>,
    zero_distance: Length,
    zero_elevation_offset: Length,
    zero_windage_offset: Length,
    zero_tolerance: Length,
    elevation_adjustment: Angle,
    windage_adjustment: Angle,
    count: u64,
}
impl IterFindAdjustments<'_> {
    fn muzzle_pitch(&self) -> Numeric {
        self.sim.muzzle_pitch.to_radians().to_num()
    }
    fn muzzle_yaw(&self) -> Numeric {
        self.sim.muzzle_yaw.to_radians().to_num()
    }
    fn elevation_adjustment(&self) -> Numeric {
        self.elevation_adjustment.to_radians().to_num()
    }
    fn windage_adjustment(&self) -> Numeric {
        self.windage_adjustment.to_radians().to_num()
    }
}
// This never returns None - it returns Some(Result) which can indicate failure instead
// This is just to capture reason why iteration stopped
impl<'s> Iterator for IterFindAdjustments<'s> {
    type Item = Result<(Angle, Angle, Length, Length)>;
    fn next(&mut self) -> Option<Self::Item> {
        // Previous pitch/yaw values to ensure angles are changing
        let &mut Self {
            sim:
                &mut super::Simulation {
                    muzzle_pitch,
                    muzzle_yaw,
                    ..
                },
            ..
        } = self;
        let muzzle_pitch = muzzle_pitch.to_radians().to_num();
        let muzzle_yaw = muzzle_yaw.to_radians().to_num();

        self.count += 1;
        self.sim.muzzle_pitch = Angle::Radians(self.muzzle_pitch() + self.elevation_adjustment());
        self.sim.muzzle_yaw = Angle::Radians(self.muzzle_yaw() + self.windage_adjustment());

        // Ensure angle is changing from previous value - may not for really small floats
        if true
            && self.muzzle_pitch() == muzzle_pitch
            && self.muzzle_yaw() == muzzle_yaw
            // Ignore first time, since both should be still be 0.0 at this point
            && self.count != 1
        {
            // dbg!((
            //     self.count,
            //     self.elevation_adjustment.to_degrees(),
            //     muzzle_pitch.to_degrees(),
            //     self.elevation_adjustment.to_degrees());
            Some(Err(Error::new(ErrorKind::AngleNotChanging(
                self.count,
                self.muzzle_pitch(),
            ))))
        } else if true
            && self.muzzle_pitch() >= DEG_45
            && self.muzzle_pitch() <= -DEG_90
            && self.muzzle_yaw() >= DEG_90
            && self.muzzle_yaw() <= -DEG_90
        {
            // dbg!((self.count, self.sim.muzzle_pitch.to_degrees()));
            Some(Err(Error::new(ErrorKind::AngleRange(
                self.count,
                self.muzzle_pitch(),
            ))))
        } else if let Some(packet) = self
            .sim
            .into_iter()
            .fuse()
            .find(|p| p.relative_position().x >= self.zero_distance.to_meters().to_num())
        {
            self.elevation_adjustment =
                packet.offset_vertical_moa(self.zero_elevation_offset, self.zero_tolerance);
            self.windage_adjustment =
                packet.offset_horizontal_moa(self.zero_windage_offset, self.zero_tolerance);
            // dbg!((self.muzzle_pitch(), self.muzzle_yaw()));
            // eprintln!("");
            Some(Ok((
                self.sim.muzzle_pitch,
                self.sim.muzzle_yaw,
                Length::Meters(packet.relative_position().y),
                Length::Meters(packet.relative_position().z),
            )))
        } else {
            // dbg!((self.count, self.sim.muzzle_pitch.to_degrees()));
            Some(Err(Error::new(ErrorKind::TerminalVelocity(
                self.count,
                self.muzzle_pitch(),
            ))))
        }
    }
}

// I would expect lifetime elision to work here
// but it currently does not
//
// impl super::Simulation<'_>
//
// note: ...so that the expression is assignable:
//           expected model::point_mass::zero::IterFindElevation<'_>
//              found model::point_mass::zero::IterFindElevation<'_>
// note: ...so that the expression is assignable:
//           expected &mut model::point_mass::Simulation<'_>
//              found &mut model::point_mass::Simulation<'_>
//
impl<'s> super::Simulation<'s> {
    fn find_adjustments(
        &'s mut self,
        zero_distance: Length,
        zero_elevation_offset: Length,
        zero_windage_offset: Length,
        zero_tolerance: Length,
    ) -> IterFindAdjustments {
        IterFindAdjustments {
            sim: self,
            zero_distance,
            zero_elevation_offset,
            zero_windage_offset,
            zero_tolerance,
            elevation_adjustment: Angle::Radians(0.0),
            windage_adjustment: Angle::Radians(0.0),
            count: 0u64,
        }
    }
    // Much more practical zeroing algorithm.  Just run flat simulation, then look at moa, and adjust
    // by that number - it's usually pretty close to the adjustment needed, so simulation only needs to be
    // ran once for most practical inputs.  Can also handle larger ranges, and will just continue to re-adjust
    // until tolerance is met.  Since MOA adjustment is always a positive number, this is probably broken for some inputs
    // This should also work for windage adjustments as well
    pub fn zero(
        &'s mut self,
        zero_distance: Length,
        zero_elevation_offset: Length,
        zero_windage_offset: Length,
        zero_tolerance: Length,
    ) -> Result<(Angle, Angle)> {
        self.find_adjustments(
            zero_distance,
            zero_elevation_offset,
            zero_windage_offset,
            zero_tolerance,
        )
        .find_map(|result| match result {
            Ok((pitch, yaw, elevation, windage)) => {
                let zero_elevation_offset = zero_elevation_offset.to_meters().to_num();
                let zero_windage_offset = zero_windage_offset.to_meters().to_num();
                let zero_tolerance = zero_tolerance.to_meters().to_num();
                let elevation = elevation.to_meters().to_num();
                let windage = windage.to_meters().to_num();

                if true
                    && elevation >= (zero_elevation_offset - zero_tolerance)
                    && elevation <= (zero_elevation_offset + zero_tolerance)
                    && windage >= (zero_windage_offset - zero_tolerance)
                    && windage <= (zero_windage_offset + zero_tolerance)
                {
                    Some(Ok((pitch, yaw)))
                } else {
                    None
                }
            }
            Err(err) => Some(Err(err)),
        })
        .unwrap()
    }
}
