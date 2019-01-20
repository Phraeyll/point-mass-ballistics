use crate::util::*;

use super::*;

// This angle will trace the longest possible trajectory for a projectile (45 degrees)
fn max_pitch() -> Numeric {
    Angle::Radians(FRAC_PI_4).to_radians().to_num()
}
// Should never try to yaw more than 90 degrees, probably not a necessary check
fn max_yaw() -> Numeric {
    Angle::Radians(FRAC_PI_2).to_radians().to_num()
}

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
impl<'s> Iterator for IterFindAdjustments<'s> {
    type Item = Result<(Angle, Angle, Length, Length), &'static str>;
    fn next(&mut self) -> Option<Self::Item> {
        let &mut Self {
            sim:
                &mut super::Simulation {
                    muzzle_pitch,
                    muzzle_yaw,
                    ..
                },
            ..
        } = self;

        self.count += 1;
        self.sim.muzzle_pitch = Angle::Radians(
            self.sim.muzzle_pitch.to_radians().to_num()
                + self.elevation_adjustment.to_radians().to_num(),
        );
        self.sim.muzzle_yaw = Angle::Radians(
            self.sim.muzzle_yaw.to_radians().to_num()
                + self.windage_adjustment.to_radians().to_num(),
        );

        if true
            && self.sim.muzzle_pitch.to_radians().to_num() == muzzle_pitch.to_radians().to_num()
            && self.sim.muzzle_yaw.to_radians().to_num() == muzzle_yaw.to_radians().to_num()
            && self.sim.muzzle_pitch.to_radians().to_num() != 0.0 // Ignore first time
            && self.sim.muzzle_yaw.to_radians().to_num() != 0.0
        // Ignore first time
        {
            // dbg!((
            //     self.count,
            //     self.elevation_adjustment.to_degrees(),
            //     muzzle_pitch.to_degrees(),
            //     self.elevation_adjustment.to_degrees()
            // ));
            Some(Err("Angle not changing, cannot zero at this range"))
        } else if true
            && self.sim.muzzle_pitch.to_radians().to_num() >= max_pitch()
            && self.sim.muzzle_yaw.to_radians().to_num() >= max_yaw()
            && self.sim.muzzle_yaw.to_radians().to_num() <= -max_yaw()
        {
            // dbg!((self.count, self.sim.muzzle_pitch.to_degrees()));
            Some(Err("Maximum angle reached, cannot zero at this range"))
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
            // dbg!((self.sim.muzzle_pitch, self.sim.muzzle_yaw));
            // eprintln!("");
            Some(Ok((
                self.sim.muzzle_pitch,
                self.sim.muzzle_yaw,
                Length::Meters(packet.relative_position().y),
                Length::Meters(packet.relative_position().z),
            )))
        } else {
            // dbg!((self.count, self.sim.muzzle_pitch.to_degrees()));
            Some(Err("Terminal velocity reached, cannot zero at this range"))
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
    ) -> Result<(Angle, Angle), &str> {
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
                    && elevation > (zero_elevation_offset - zero_tolerance)
                    && elevation < (zero_elevation_offset + zero_tolerance)
                    && windage > (zero_windage_offset - zero_tolerance)
                    && windage < (zero_windage_offset + zero_tolerance)
                {
                    Some(Ok((pitch, yaw)))
                } else {
                    None
                }
            }
            Err(err) => Some(Err(err)),
        })
        .unwrap_or(Err("Can this ever be reached?"))
    }
}
