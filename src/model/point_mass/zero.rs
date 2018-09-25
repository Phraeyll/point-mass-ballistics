use approx::relative_eq;

use crate::util::{conversions::*, Numeric, FRAC_PI_4};

impl<'mc> super::Simulation<'mc> {
    // Find muzzle angle to achieve 0 drop at specified distance, relative to scope height
    pub(crate) fn zero(&mut self, zero_distance: Length) -> Result<Numeric, String> {
        // This angle will trace the longest possible trajectory for a projectile (45 degrees)
        const MAX_ANGLE: Numeric = FRAC_PI_4;
        // Start with maximum angle to allow for zeroing at longer distances
        let mut angle = MAX_ANGLE;
        loop {
            let last_muzzle_pitch: Numeric = self.muzzle_pitch;
            self.muzzle_pitch += angle;
            if self.muzzle_pitch > MAX_ANGLE {
                break Err(format!(
                    "Can never 'zero' at this range: {}",
                    Numeric::from(zero_distance)
                ));
            }
            if self.muzzle_pitch == last_muzzle_pitch {
                break Err(format!(
                    "Issue with floating points, pitch: {} by angle: {} not changing during 'zero'",
                    last_muzzle_pitch, angle,
                ));
            }
            // Find drop at distance, need way to break if we never zero_distance
            let drop = self
                .iter()
                .find(|p| p.relative_position().x > Numeric::from(zero_distance.to_meters()))
                .unwrap()
                .relative_position()
                .y;
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
