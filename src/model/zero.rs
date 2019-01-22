use crate::util::*;
use crate::model::builder::*;

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
    fn pitch(&self) -> Numeric {
        self.sim.angles.pitch.to_radians().to_num()
    }
    fn yaw(&self) -> Numeric {
        self.sim.angles.yaw.to_radians().to_num()
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
                    angles: super::Angles { pitch, yaw, .. },
                    ..
                },
            ..
        } = self;
        let pitch = pitch.to_radians().to_num();
        let yaw = yaw.to_radians().to_num();

        self.count += 1;
        self.sim.angles.pitch = Angle::Radians(self.pitch() + self.elevation_adjustment());
        self.sim.angles.yaw = Angle::Radians(self.yaw() + self.windage_adjustment());

        // Ensure angle is changing from previous value - may not for really small floats
        if true
            && self.pitch() == pitch
            && self.yaw() == yaw
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
                self.pitch(),
            ))))
        } else if true
            && self.pitch() >= DEG_45
            && self.pitch() <= -DEG_90
            && self.yaw() >= DEG_90
            && self.yaw() <= -DEG_90
        {
            // dbg!((self.count, self.sim.muzzle_pitch.to_degrees()));
            Some(Err(Error::new(ErrorKind::AngleRange(
                self.count,
                self.pitch(),
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
                self.sim.angles.pitch,
                self.sim.angles.yaw,
                Length::Meters(packet.relative_position().y),
                Length::Meters(packet.relative_position().z),
            )))
        } else {
            // dbg!((self.count, self.sim.muzzle_pitch.to_degrees()));
            Some(Err(Error::new(ErrorKind::TerminalVelocity(
                self.count,
                self.pitch(),
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
    ) -> Result<Angles> {
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
                    Some(Ok(Angles {
                        pitch,
                        yaw,
                        roll: Angle::Radians(0.0),
                    }))
                } else {
                    None
                }
            }
            Err(err) => Some(Err(err)),
        })
        .unwrap()
    }
}
