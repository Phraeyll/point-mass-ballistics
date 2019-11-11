use crate::{
    output::Packet, simulation::Scope, util::*, Error, ErrorKind, Measurements, Result, Simulation,
};

// This angle will trace the longest possible trajectory for a projectile (45 degrees)
const DEG_45: Angle = Angle::Radians(FRAC_PI_4);
// Should never try to yaw more than 90 degrees, probably not a necessary check
// Also should never try to pitch this low - not sure if this ever happens in practice
const DEG_90: Angle = Angle::Radians(FRAC_PI_2);

struct IterFindAdjustments<'t, F, E, W>
where
    F: Fn(&Packet) -> bool,
    E: Fn(&Packet) -> Angle,
    W: Fn(&Packet) -> Angle,
{
    sim: &'t mut Simulation<'t>,

    finder: F,
    elevation_adjuster: E,
    windage_adjuster: W,

    elevation_adjustment: Angle,
    windage_adjustment: Angle,
    count: u64,
}
// This never returns None - it returns Some(Result) which can indicate failure instead
// This is just to capture reason why iteration stopped
impl<F, E, W> Iterator for IterFindAdjustments<'_, F, E, W>
where
    F: Fn(&Packet) -> bool,
    E: Fn(&Packet) -> Angle,
    W: Fn(&Packet) -> Angle,
{
    type Item = Result<(Angle, Angle, Numeric, Numeric)>;
    fn next(&mut self) -> Option<Self::Item> {
        // Previous pitch/yaw values to ensure angles are changing
        let &mut Self {
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
            Some(Err(Error::new(ErrorKind::AngleNotChanging {
                count,
                pitch,
                yaw,
            })))
        } else if (pitch >= DEG_45 && pitch <= -DEG_90) || (yaw >= DEG_90 && yaw <= -DEG_90) {
            Some(Err(Error::new(ErrorKind::AngleRange { count, pitch, yaw })))
        } else if let Some(packet) = self.sim.into_iter().fuse().find(&self.finder) {
            self.elevation_adjustment = (self.elevation_adjuster)(&packet);
            self.windage_adjustment = (self.windage_adjuster)(&packet);
            let elevation = packet.relative_position().y;
            let windage = packet.relative_position().z;
            Some(Ok((pitch, yaw, elevation, windage)))
        } else {
            Some(Err(Error::new(ErrorKind::TerminalVelocity {
                count,
                pitch,
                yaw,
            })))
        }
    }
}

impl<'t> Simulation<'t> {
    fn find_adjustments<F, E, W>(
        &'t mut self,
        finder: F,
        elevation_adjuster: E,
        windage_adjuster: W,
    ) -> IterFindAdjustments<'t, F, E, W>
    where
        F: Fn(&Packet) -> bool,
        E: Fn(&Packet) -> Angle,
        W: Fn(&Packet) -> Angle,
    {
        IterFindAdjustments {
            sim: self,

            finder,
            elevation_adjuster,
            windage_adjuster,

            elevation_adjustment: Angle::Minutes(0.0),
            windage_adjustment: Angle::Minutes(0.0),
            count: 0u64,
        }
    }
}
impl<'t> Simulation<'t> {
    // Much more practical zeroing algorithm.  Just run flat simulation, then look at moa, and adjust
    // by that number - it's usually pretty close to the adjustment needed, so simulation only needs to be
    // ran once for most practical inputs.  Can also handle larger ranges, and will just continue to re-adjust
    // until tolerance is met.  Since MOA adjustment is always a positive number, this is probably broken for some inputs
    // This should also work for windage adjustments as well
    pub fn find_zero_angles(
        &'t mut self,
        distance: Numeric,
        elevation_offset: Numeric,
        windage_offset: Numeric,
        tolerance: Numeric,
    ) -> Result<(Numeric, Numeric)> {
        let distance = Length::Yards(distance).to_meters().to_num();
        let elevation_offset = Length::Inches(elevation_offset).to_meters().to_num();
        let windage_offset = Length::Inches(windage_offset).to_meters().to_num();
        let tolerance = Length::Inches(tolerance).to_meters().to_num();

        let (pitch, yaw, _, _) = self
            .find_adjustments(
                { |p: &Packet| p.relative_position().x >= distance },
                { |p: &Packet| p.offset_vertical_moa(elevation_offset, tolerance) },
                { |p: &Packet| p.offset_horizontal_moa(windage_offset, tolerance) },
            )
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
            .unwrap()?; // The iterator always returns Some - unwrap to inner result, then handle with "?"
        Ok((pitch.to_minutes().to_num(), yaw.to_minutes().to_num()))
    }
}
