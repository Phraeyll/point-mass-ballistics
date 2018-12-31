use approx::relative_eq;

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
        let muzzle_pitch = self.iter.muzzle_pitch;
        let drop = self.drop;
        let zero_distance = Numeric::from(self.iter.zero_distance.to_meters());

        // Increment/decrement pitch before iteration below
        self.iter.muzzle_pitch += self.angle;

        // Maximum angle or muzzle_pitch not changing due to very small angle (floating point limitation)
        if self.iter.muzzle_pitch > MAX_ANGLE || self.iter.muzzle_pitch == muzzle_pitch {
            return None;
        }
        // Find height in meters relative to zero, given pitch
        if let Some(projectile) = self
            .iter
            .iter()
            .find(|p| p.relative_position().x >= zero_distance)
        {
            self.drop = projectile.relative_position().y;
        } else {
            // Terminal velocity reached
            return None;
        };
        // Change direction and angle by 1/2 if
        // above(positive drop) and going   up(positive angle) or
        // below(negative drop) and going down(negative angle)
        if self.angle.is_sign_positive() ^ self.drop.is_sign_negative() {
            self.angle *= -1.0;
        }
        self.angle /= 2.0;

        Some((muzzle_pitch, drop))
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
        if let Some((muzzle_pitch, _)) = self.zero_iter().find(|(_, drop)| relative_eq!(*drop, 0.0))
        {
            Ok(muzzle_pitch)
        } else {
            Err(String::from("Cannot zero for this range"))
        }
    }
}
