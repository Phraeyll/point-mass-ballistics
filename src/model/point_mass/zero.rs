use crate::util::{Numeric, FRAC_PI_4};

// This angle will trace the longest possible trajectory for a projectile (45 degrees)
const MAX_ANGLE: Numeric = FRAC_PI_4;

struct IterZero<'s> {
    iter: &'s mut super::Simulation<'s>,
    angle: Numeric,
    drop: Numeric,
}
impl<'s> Iterator for IterZero<'s> {
    type Item = (Numeric, Numeric);
    fn next(&mut self) -> Option<Self::Item> {
        // Keep previous value to check if pitch changes
        let muzzle_pitch = self.iter.muzzle_pitch;

        // Increment/decrement pitch before iteration below
        self.iter.muzzle_pitch += self.angle;

        // Maximum angle or muzzle_pitch not changing due to very small angle (floating point limitation)
        if self.iter.muzzle_pitch > MAX_ANGLE {
            println!("Greater than MAX_ANGLE: {}", MAX_ANGLE);
            return None;
        }
        // This should probably not happen in practice, only for very small values close to 0
        if self.iter.muzzle_pitch == muzzle_pitch {
            println!(
                "Floating Point Err\nbfore: {:+.64}\nangle: {:+.64}\nafter: {:+.64}\ndrop: {:+.64}",
                muzzle_pitch, self.angle, self.iter.muzzle_pitch, self.drop
            );
            return None;
        }
        // Find height in meters relative to zero, given pitch
        if let Some(p) = self
            .iter
            .iter()
            .find(|p| p.relative_position().x >= Numeric::from(self.iter.zero_distance.to_meters()))
        {
            self.drop = p.relative_position().y;
        } else {
            // Terminal velocity reached
            return None;
        };
        // Change direction if
        // above(positive drop) and going   up(positive angle) or
        // below(negative drop) and going down(negative angle)
        if self.angle.is_sign_positive() ^ self.drop.is_sign_negative() {
            self.angle *= -1.0;
        }
        // Always reduce angle on next iteration, converging towards drop = 0
        self.angle /= 2.0;

        Some((self.iter.muzzle_pitch, self.drop))
    }
}

impl<'s> super::Simulation<'s> {
    fn zero_iter(&'s mut self) -> IterZero {
        // This angle will trace the longest possible trajectory for a projectile (45 degrees)
        // Start with maximum angle to allow for zeroing at longer distances
        IterZero {
            iter: self,
            angle: MAX_ANGLE,
            drop: -1.0,
        }
    }
}
impl<'s> super::Simulation<'s> {
    // Find muzzle angle to achieve 0 drop at specified distance, relative to scope height
    pub(crate) fn zero(&'s mut self) -> Result<Numeric, String> {
        if let Some((muzzle_pitch, _)) = self
            .zero_iter()
            .find(|(_, drop)| *drop >= -0.000_001 && *drop <= 0.000_001)
        {
            Ok(muzzle_pitch)
        } else {
            Err(String::from("Cannot zero for this range"))
        }
    }
}
