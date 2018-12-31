use approx::relative_eq;

use crate::util::{Numeric, FRAC_PI_4};

impl super::Simulation<'_> {
    // Find muzzle angle to achieve 0 drop at specified distance, relative to scope height
    pub(crate) fn zero(&mut self) -> Result<Numeric, String> {
        // This angle will trace the longest possible trajectory for a projectile (45 degrees)
        const MAX_ANGLE: Numeric = FRAC_PI_4;
        // Start with maximum angle to allow for zeroing at longer distances
        let mut angle = MAX_ANGLE;
        loop {
            let muzzle_pitch: Numeric = self.muzzle_pitch;
            self.muzzle_pitch += angle;
            if self.muzzle_pitch > MAX_ANGLE {
                break Err(format!(
                    "Can never 'zero' at this range: {}",
                    Numeric::from(self.zero_distance)
                ));
            }
            if self.muzzle_pitch == muzzle_pitch {
                break Err(format!(
                    "Issue with floating points, pitch: {} by angle: {} not changing during 'zero'",
                    muzzle_pitch, angle,
                ));
            }
            let drop = if let Some(projectile) = self
                .iter()
                .find(|p| p.relative_position().x >= Numeric::from(self.zero_distance.to_meters()))
            {
                projectile.relative_position().y
            } else {
                break Err(format!(
                    "Reach terminal velocity 'zeroing' for this range: {}",
                    Numeric::from(self.zero_distance)
                ));
            };
            // Quit once zero point is found, once drop is equal to zero
            if relative_eq!(drop, 0.0) {
                break Ok(self.muzzle_pitch);
            }
            // If in the following states (xor), change direction by flipping angle sign
            // true, false || false, true
            // up,   above || down,  below
            if angle.is_sign_positive() ^ drop.is_sign_negative() {
                angle *= -1.0;
            }
            // Reduce angle before next iteration, trying to converge on zero point
            angle /= 2.0;
        }
    }
}
