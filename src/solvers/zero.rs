use crate::{
    simulation::{Scope, Simulation},
    util::*,
    Error, ErrorKind, Measurements, Result,
};

// This angle will trace the longest possible trajectory for a projectile (45 degrees)
const DEG_45: Angle = Angle::Radians(FRAC_PI_4);
// Should never try to yaw more than 90 degrees, probably not a necessary check
// Also should never try to pitch this low - not sure if this ever happens in practice
const DEG_90: Angle = Angle::Radians(FRAC_PI_2);

struct IterFindAdjustments<'s> {
    sim: &'s mut Simulation,
    distance: Numeric,
    elevation_offset: Numeric,
    windage_offset: Numeric,
    tolerance: Numeric,
    elevation_adjustment: Angle,
    windage_adjustment: Angle,
    count: u64,
}
// This never returns None - it returns Some(Result) which can indicate failure instead
// This is just to capture reason why iteration stopped
impl Iterator for IterFindAdjustments<'_> {
    type Item = Result<(Angle, Angle, Numeric, Numeric)>;
    fn next(&mut self) -> Option<Self::Item> {
        // Previous pitch/yaw values to ensure angles are changing
        let &mut Self {
            distance,
            elevation_offset,
            windage_offset,
            tolerance,
            sim:
                &mut Simulation {
                    scope:
                        Scope {
                            pitch: prev_pitch,
                            yaw: prev_yaw,
                            ..
                        },
                    ..
                },
            ..
        } = self;

        // Capture current adjustments/count for use during checks later
        // Cannot return pitch/yaw here, as zeroth iteration doesn't have
        // meaningful value for elevation/windage - really have to run iter at least once
        self.count += 1;
        self.sim.scope.pitch += self.elevation_adjustment;
        self.sim.scope.yaw += self.windage_adjustment;
        let count = self.count;
        let pitch = self.sim.scope.pitch;
        let yaw = self.sim.scope.yaw;

        // Ensure angle is changing from previous value - may not for really small floats
        if true
            && pitch == prev_pitch
            && yaw == prev_yaw
            // Ignore first time, since both should be still be 0.0 at this point
            && count != 1
        {
            // dbg!((
            //     self.count,
            //     self.elevation_adjustment.to_degrees(),
            //     muzzle_pitch.to_degrees(),
            //     self.elevation_adjustment.to_degrees());
            Some(Err(Error::new(ErrorKind::AngleNotChanging {
                count,
                pitch,
                yaw,
            })))
        } else if pitch >= DEG_45 && pitch <= -DEG_90 && yaw >= DEG_90 && yaw <= -DEG_90 {
            // dbg!((self.count, self.sim.muzzle_pitch.to_degrees()));
            Some(Err(Error::new(ErrorKind::AngleRange { count, pitch, yaw })))
        } else if let Some(packet) = self
            .sim
            .into_iter()
            .fuse()
            .find(|p| p.relative_position().x >= distance)
        {
            self.elevation_adjustment = packet.offset_vertical_moa(elevation_offset, tolerance);
            self.windage_adjustment = packet.offset_horizontal_moa(windage_offset, tolerance);
            let elevation = packet.relative_position().y;
            let windage = packet.relative_position().z;
            // dbg!((self.muzzle_pitch(), self.muzzle_yaw()));
            // eprintln!("");
            Some(Ok((pitch, yaw, elevation, windage)))
        } else {
            // dbg!((self.count, self.sim.muzzle_pitch.to_degrees()));
            Some(Err(Error::new(ErrorKind::TerminalVelocity {
                count,
                pitch,
                yaw,
            })))
        }
    }
}

impl Simulation {
    fn find_adjustments(
        &mut self,
        distance: Numeric,
        elevation_offset: Numeric,
        windage_offset: Numeric,
        tolerance: Numeric,
    ) -> IterFindAdjustments {
        IterFindAdjustments {
            sim: self,
            distance,
            elevation_offset,
            windage_offset,
            tolerance,
            elevation_adjustment: Angle::Minutes(0.0),
            windage_adjustment: Angle::Minutes(0.0),
            count: 0u64,
        }
    }
    // Much more practical zeroing algorithm.  Just run flat simulation, then look at moa, and adjust
    // by that number - it's usually pretty close to the adjustment needed, so simulation only needs to be
    // ran once for most practical inputs.  Can also handle larger ranges, and will just continue to re-adjust
    // until tolerance is met.  Since MOA adjustment is always a positive number, this is probably broken for some inputs
    // This should also work for windage adjustments as well
    pub fn try_mut_zero(
        &mut self,
        distance: Numeric,
        elevation_offset: Numeric,
        windage_offset: Numeric,
        tolerance: Numeric,
    ) -> Result<()> {
        let Scope {
            pitch: prev_pitch,
            yaw: prev_yaw,
            ..
        } = self.scope;
        self.scope = Scope {
            pitch: Angle::Minutes(0.0),
            yaw: Angle::Minutes(0.0),
            ..self.scope
        };
        let distance = Length::Yards(distance).to_meters().to_num();
        let elevation_offset = Length::Inches(elevation_offset).to_meters().to_num();
        let windage_offset = Length::Inches(windage_offset).to_meters().to_num();
        let tolerance = Length::Inches(tolerance).to_meters().to_num();
        self.find_adjustments(distance, elevation_offset, windage_offset, tolerance)
            .find_map(|result| match result {
                Ok((_, _, elevation, windage)) => {
                    if true
                        && elevation >= (elevation_offset - tolerance)
                        && elevation <= (elevation_offset + tolerance)
                        && windage >= (windage_offset - tolerance)
                        && windage <= (windage_offset + tolerance)
                    {
                        Some(result)
                    } else {
                        None
                    }
                }
                err @ Err(_) => Some(err),
            })
            .unwrap() // Always unwraps Some - None above indicates continuing iteration in find_map
            .map(|(pitch, yaw, _, _)| {
                self.scope.pitch = pitch + prev_pitch;
                self.scope.yaw = yaw + prev_yaw;
            })
    }
}
