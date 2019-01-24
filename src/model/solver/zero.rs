use crate::model::core::{Scope, Simulation};
use crate::util::*;

// This angle will trace the longest possible trajectory for a projectile (45 degrees)
const DEG_45: Angle = Angle::Radians(FRAC_PI_4);
// Should never try to yaw more than 90 degrees, probably not a necessary check
// Also should never try to pitch this low - not sure if this ever happens in practice
const DEG_90: Angle = Angle::Radians(FRAC_PI_2);

struct IterFindAdjustments<'s> {
    sim: &'s mut Simulation,
    zero_distance: Numeric,
    zero_elevation_offset: Numeric,
    zero_windage_offset: Numeric,
    zero_tolerance: Numeric,
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
            sim:
                &mut Simulation {
                    scope: Scope { pitch, yaw, .. },
                    ..
                },
            ..
        } = self;

        self.count += 1;
        self.sim.scope.pitch += self.elevation_adjustment;
        self.sim.scope.yaw += self.windage_adjustment;

        // Ensure angle is changing from previous value - may not for really small floats
        if true
            && self.sim.scope.pitch == pitch
            && self.sim.scope.pitch == yaw
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
                self.sim.scope.pitch.to_radians().to_num(),
            ))))
        } else if true
            && self.sim.scope.pitch >= DEG_45
            && self.sim.scope.pitch <= -DEG_90
            && self.sim.scope.yaw >= DEG_90
            && self.sim.scope.yaw <= -DEG_90
        {
            // dbg!((self.count, self.sim.muzzle_pitch.to_degrees()));
            Some(Err(Error::new(ErrorKind::AngleRange(
                self.count,
                self.sim.scope.pitch.to_radians().to_num(),
            ))))
        } else if let Some(packet) = self
            .sim
            .into_iter()
            .fuse()
            .find(|p| p.relative_position().x >= self.zero_distance)
        {
            self.elevation_adjustment =
                packet.offset_vertical_moa(self.zero_elevation_offset, self.zero_tolerance);
            self.windage_adjustment =
                packet.offset_horizontal_moa(self.zero_windage_offset, self.zero_tolerance);
            // dbg!((self.muzzle_pitch(), self.muzzle_yaw()));
            // eprintln!("");
            Some(Ok((
                self.sim.scope.pitch,
                self.sim.scope.yaw,
                packet.relative_position().y,
                packet.relative_position().z,
            )))
        } else {
            // dbg!((self.count, self.sim.muzzle_pitch.to_degrees()));
            Some(Err(Error::new(ErrorKind::TerminalVelocity(
                self.count,
                self.sim.scope.pitch.to_radians().to_num(),
            ))))
        }
    }
}

impl Simulation {
    fn find_adjustments(
        &mut self,
        zero_distance: Numeric,
        zero_elevation_offset: Numeric,
        zero_windage_offset: Numeric,
        zero_tolerance: Numeric,
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
        mut self,
        zero_distance: Numeric,
        zero_elevation_offset: Numeric,
        zero_windage_offset: Numeric,
        zero_tolerance: Numeric,
    ) -> Result<Simulation> {
        let zero_distance = Length::Yards(zero_distance).to_meters().to_num();
        let zero_elevation_offset = Length::Inches(zero_elevation_offset).to_meters().to_num();
        let zero_windage_offset = Length::Inches(zero_windage_offset).to_meters().to_num();
        let zero_tolerance = Length::Inches(zero_tolerance).to_meters().to_num();
        self.find_adjustments(
            zero_distance,
            zero_elevation_offset,
            zero_windage_offset,
            zero_tolerance,
        )
        .find_map(|result| match result {
            Ok((_, _, elevation, windage)) => {
                if true
                    && elevation >= (zero_elevation_offset - zero_tolerance)
                    && elevation <= (zero_elevation_offset + zero_tolerance)
                    && windage >= (zero_windage_offset - zero_tolerance)
                    && windage <= (zero_windage_offset + zero_tolerance)
                {
                    Some(result)
                } else {
                    None
                }
            }
            result @ Err(_) => Some(result),
        })
        .unwrap() // Always unwraps Some - None above indicates continuing iteration in find_map
        .map(|(pitch, yaw, _, _)| {
            self.scope = Scope {
                pitch,
                yaw,
                ..self.scope
            }; // Keep roll same, not adjusted during zeroing
            self
        })
    }
    pub fn try_mut_zero(
        &mut self,
        zero_distance: Numeric,
        zero_elevation_offset: Numeric,
        zero_windage_offset: Numeric,
        zero_tolerance: Numeric,
    ) -> Result<()> {
        let zero_distance = Length::Yards(zero_distance).to_meters().to_num();
        let zero_elevation_offset = Length::Inches(zero_elevation_offset).to_meters().to_num();
        let zero_windage_offset = Length::Inches(zero_windage_offset).to_meters().to_num();
        let zero_tolerance = Length::Inches(zero_tolerance).to_meters().to_num();
        self.find_adjustments(
            zero_distance,
            zero_elevation_offset,
            zero_windage_offset,
            zero_tolerance,
        )
        .find_map(|result| match result {
            Ok((_, _, elevation, windage)) => {
                if true
                    && elevation >= (zero_elevation_offset - zero_tolerance)
                    && elevation <= (zero_elevation_offset + zero_tolerance)
                    && windage >= (zero_windage_offset - zero_tolerance)
                    && windage <= (zero_windage_offset + zero_tolerance)
                {
                    Some(result)
                } else {
                    None
                }
            }
            result @ Err(_) => Some(result),
        })
        .unwrap() // Always unwraps Some - None above indicates continuing iteration in find_map
        .map(|(pitch, yaw, _, _)| {
            self.scope = Scope {
                pitch,
                yaw,
                ..self.scope
            }; // Keep roll same, not adjusted during zeroing
        })
    }
}
