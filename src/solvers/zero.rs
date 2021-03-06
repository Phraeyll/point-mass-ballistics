use crate::{
    consts::{FRAC_PI_2, FRAC_PI_4},
    error::{Error, Result},
    my_quantity,
    output::Measurements,
    output::Packet,
    projectiles::Projectile,
    simulation::Scope,
    simulation::Simulation,
    units::{angle, radian, Angle, Length, MyQuantity},
};

// This angle will trace the longest possible trajectory for a projectile (45 degrees)
const DEG_45: MyQuantity<angle::Dimension> = my_quantity!(FRAC_PI_4);

// Should never try to yaw more than 90 degrees, probably not a necessary check
// Also should never try to pitch this low - not sure if this ever happens in practice
const DEG_90: MyQuantity<angle::Dimension> = my_quantity!(FRAC_PI_2);

struct IterFindAdjustments<'t, T, F, E, W>
where
    T: Projectile,
    F: Fn(&Packet<T>) -> bool,
    E: Fn(&Packet<T>) -> Angle,
    W: Fn(&Packet<T>) -> Angle,
{
    sim: &'t mut Simulation<T>,

    finder: F,
    elevation_adjuster: E,
    windage_adjuster: W,

    elevation_adjustment: Angle,
    windage_adjustment: Angle,
    count: u64,
}

// This never returns None - it returns Some(Result) which can indicate failure instead
// This is just to capture reason why iteration stopped
impl<T, F, E, W> Iterator for IterFindAdjustments<'_, T, F, E, W>
where
    T: Projectile,
    F: Fn(&Packet<T>) -> bool,
    E: Fn(&Packet<T>) -> Angle,
    W: Fn(&Packet<T>) -> Angle,
{
    type Item = Result<(Angle, Angle, Length, Length)>;
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
            Some(Err(Error::AngleNotChanging { count, pitch, yaw }))
        } else if (pitch >= DEG_45 || pitch <= -DEG_90) || (yaw >= DEG_90 || yaw <= -DEG_90) {
            Some(Err(Error::AngleRange { count, pitch, yaw }))
        } else if let Some(packet) = self.sim.into_iter().fuse().find(&self.finder) {
            self.elevation_adjustment = (self.elevation_adjuster)(&packet);
            self.windage_adjustment = (self.windage_adjuster)(&packet);
            let elevation = packet.elevation();
            let windage = packet.windage();
            Some(Ok((pitch, yaw, elevation, windage)))
        } else {
            Some(Err(Error::TerminalVelocity { count, pitch, yaw }))
        }
    }
}

impl<'t, T> Simulation<T>
where
    T: Projectile,
{
    fn find_adjustments<F, E, W>(
        &'t mut self,
        finder: F,
        elevation_adjuster: E,
        windage_adjuster: W,
    ) -> IterFindAdjustments<'t, T, F, E, W>
    where
        F: Fn(&Packet<T>) -> bool,
        E: Fn(&Packet<T>) -> Angle,
        W: Fn(&Packet<T>) -> Angle,
    {
        IterFindAdjustments {
            sim: self,

            finder,
            elevation_adjuster,
            windage_adjuster,

            elevation_adjustment: Angle::new::<radian>(0.0),
            windage_adjustment: Angle::new::<radian>(0.0),
            count: 0u64,
        }
    }
}

impl<T> Simulation<T>
where
    T: Projectile,
{
    // Much more practical zeroing algorithm.  Just run flat simulation, then look at moa, and adjust
    // by that number - it's usually pretty close to the adjustment needed, so simulation only needs to be
    // ran once for most practical inputs.  Can also handle larger ranges, and will just continue to re-adjust
    // until tolerance is met.  Since MOA adjustment is always a positive number, this is probably broken for some inputs
    // This should also work for windage adjustments as well
    pub fn find_zero_angles(
        &mut self,
        distance: Length,
        elevation_offset: Length,
        windage_offset: Length,
        tolerance: Length,
    ) -> Result<(Angle, Angle)> {
        let (pitch, yaw, _, _) = self
            .find_adjustments(
                |p: &Packet<T>| p.distance() >= distance,
                |p: &Packet<T>| p.offset_vertical_angle(elevation_offset, tolerance),
                |p: &Packet<T>| p.offset_horizontal_angle(windage_offset, tolerance),
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
        Ok((pitch, yaw))
    }
}
