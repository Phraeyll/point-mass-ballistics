use crate::util::{conversions::*, Numeric, FRAC_PI_2, FRAC_PI_4};

// This angle will trace the longest possible trajectory for a projectile (45 degrees)
const MAX_ANGLE: Numeric = FRAC_PI_4;

struct IterZero<'s> {
    sim: &'s mut super::Simulation<'s>,
    angle: Numeric,
    drop: Numeric,
    count: u64,
}

impl<'s> Iterator for IterZero<'s> {
    type Item = (Numeric, Numeric);
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        // Keep previous value to check if pitch changes
        let &mut Self {
            sim: &mut super::Simulation { muzzle_pitch, .. },
            count,
            ..
        } = self;

        // Change direction if
        // above(positive drop) and going   up(positive angle) or
        // below(negative drop) and going down(negative angle)
        if self.angle.is_sign_positive() ^ self.drop.is_sign_negative() {
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
                "Floating Point Err\nbfore: {:+.64}\nangle: {:+.64}\nafter: {:+.64}\ndrop: {:+.64}\ncount: {}\npitch: {:.2}",
                muzzle_pitch, self.angle, self.sim.muzzle_pitch, self.drop, count, deg
            );
            None
        } else if let Some(p) = self
            // Find height in meters relative to zero, given pitch
            .sim
            .into_iter()
            .find(|p| p.relative_position().x >= self.sim.zero_distance.to_meters().to_num())
        {
            self.drop = p.relative_position().y;
            Some((
                self.sim.muzzle_pitch,
                Length::Meters(self.drop).to_inches().to_num(),
            ))
        } else {
            // Terminal velocity reached
            println!("count: {}", count);
            None
        }
    }
}

impl<'s> super::Simulation<'s> {
    fn zero_iter(&'s mut self) -> IterZero {
        // This angle will trace the longest possible trajectory for a projectile (45 degrees)
        // Start with maximum angle to allow for zeroing at longer distances
        IterZero {
            sim: self,
            angle: FRAC_PI_2,
            drop: -1.0,
            count: 0u64,
        }
    }
    // Find muzzle angle to achieve 0 drop at specified distance, relative to scope height
    pub(crate) fn zero(&'s mut self) -> Result<Numeric, &'static str> {
        const MAX: Numeric = 0.001;
        self.zero_iter()
            .find(|&(_, drop)| drop > -MAX && drop < MAX)
            .map_or(Err("Cannot zero for this range"), |(p, _)| Ok(p))
    }
}
