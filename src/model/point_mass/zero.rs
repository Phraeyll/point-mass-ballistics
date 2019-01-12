use crate::util::{conversions::*, Numeric, FRAC_PI_2, FRAC_PI_4};

// This angle will trace the longest possible trajectory for a projectile (45 degrees)
const MAX_ANGLE: Numeric = FRAC_PI_4;

struct IterFindElevation<'s> {
    sim: &'s mut super::Simulation<'s>,
    angle: Numeric,
    elevation: Numeric,
    zero_distance: Numeric,
    zero_offset: Numeric,
    count: u64,
}

impl<'s> Iterator for IterFindElevation<'s> {
    type Item = (Numeric, Numeric);
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        // Keep previous value to check if pitch changes
        let &mut Self {
            sim: &mut super::Simulation { muzzle_pitch, .. },
            count,
            elevation,
            ..
        } = self;
        let elevation = Length::Meters(elevation).to_inches().to_num();

        // Change direction if
        // above(positive drop) and going   up(positive angle) or
        // below(negative drop) and going down(negative angle)
        if self.angle.is_sign_positive() ^ (self.elevation < self.zero_offset) {
            self.angle *= -1.0;
        }
        // Always reduce angle on next iteration, converging towards either max(45) or min(0) degrees
        self.angle /= 2.0;

        // Increment/decrement pitch before iteration below
        self.sim.muzzle_pitch += self.angle;
        let deg = self.sim.muzzle_pitch.to_degrees();

        if self.sim.muzzle_pitch > MAX_ANGLE {
            // This only can happen on second iteration, starting at 45 degrees
            // If switched to 45/2 degrees, algorithm will converge to either 45 or 0
            // Switched back to starting at 45 degrees to allow quick break if possible
            println!(
                "Greater than MAX_ANGLE: {} at iteration: {} at pitch: {:.2}",
                MAX_ANGLE.to_degrees(),
                count,
                deg
            );
            None
        } else if self.sim.muzzle_pitch == muzzle_pitch {
            // muzzle_pitch not changing due to very small angle (floating point limitation)
            // This should probably not happen in practice, only for very small values close to 0
            println!(
                "Floating Point Err\nbfore: {:+.64}\nangle: {:+.64}\nafter: {:+.64}\nelevation: {:+.64}\ncount: {}\npitch: {:.2}",
                muzzle_pitch, self.angle, self.sim.muzzle_pitch, elevation, count, deg
            );
            None
        } else if let Some(p) = self
            // Find height in meters relative to zero, given pitch
            .sim
            .iter()
            .find(|p| p.relative_position().x >= self.zero_distance)
        {
            self.elevation = p.relative_position().y;
            Some((
                self.sim.muzzle_pitch,
                self.elevation,
            ))
        } else {
            // Terminal velocity reached
            println!("count: {}", count);
            None
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
    fn find_elevation(&'s mut self, zero_distance: Numeric, zero_offset: Numeric) -> IterFindElevation {
        // This angle will trace the longest possible trajectory for a projectile (45 degrees)
        // Start with maximum angle to allow for zeroing at longer distances
        IterFindElevation {
            sim: self,
            angle: FRAC_PI_2,
            elevation: -1.0,
            zero_distance,
            zero_offset,
            count: 0u64,
        }
    }
    // Find muzzle angle to achieve 0 drop at specified distance, relative to scope height
    pub(crate) fn zero(
        &'s mut self,
        zero_distance: Numeric,
        zero_offset: Numeric,
        zero_tolerance: Numeric,
    ) -> Result<Numeric, &'static str> {
        self.find_elevation(zero_distance, zero_offset)
            .find(|&(_, elevation)| {
                elevation > (zero_offset - zero_tolerance) && elevation < (zero_offset + zero_tolerance)
            })
            .map(|(pitch, _)| Ok(pitch))
            .expect("Cannot zero for this range")
    }
}
