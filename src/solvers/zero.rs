use crate::{
    consts::{FRAC_PI_2, FRAC_PI_4},
    error::{Error, Result},
    output::Measurements,
    output::Packet,
    physics::DragFunction,
    simulation::Scope,
    simulation::Simulation,
    units::{angle, my_quantity, Angle, ConstZero, Length, MyQuantity},
};

// This angle will trace the longest possible trajectory for a projectile (45 degrees)
const DEG_45: MyQuantity<angle::Dimension> = my_quantity!(FRAC_PI_4);

// Should never try to yaw more than 90 degrees, probably not a necessary check
// Also should never try to pitch this low - not sure if this ever happens in practice
const DEG_90: MyQuantity<angle::Dimension> = my_quantity!(FRAC_PI_2);

struct IterFindAdjustments<'a, D, F, E, W>
where
    F: Fn(&Packet<D>) -> bool,
    E: Fn(&Packet<D>) -> Angle,
    W: Fn(&Packet<D>) -> Angle,
{
    sim: &'a mut Simulation<D>,

    finder: F,
    elevation_adjuster: E,
    windage_adjuster: W,

    elevation_adjustment: Angle,
    windage_adjustment: Angle,
    count: u64,
}

// This never returns None - it returns Some(Result) which can indicate failure instead
// This is just to capture reason why iteration stopped
impl<D, F, E, W> Iterator for IterFindAdjustments<'_, D, F, E, W>
where
    D: DragFunction,
    F: Fn(&Packet<D>) -> bool,
    E: Fn(&Packet<D>) -> Angle,
    W: Fn(&Packet<D>) -> Angle,
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
        // Cannot return pitch/yaw here, as zeroth iteration doesn'a have
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
            None
        }
    }
}

impl<'a, D> Simulation<D> {
    fn find_adjustments<F, E, W>(
        &'a mut self,
        finder: F,
        elevation_adjuster: E,
        windage_adjuster: W,
    ) -> IterFindAdjustments<'a, D, F, E, W>
    where
        F: Fn(&Packet<D>) -> bool,
        E: Fn(&Packet<D>) -> Angle,
        W: Fn(&Packet<D>) -> Angle,
    {
        IterFindAdjustments {
            sim: self,

            finder,
            elevation_adjuster,
            windage_adjuster,

            elevation_adjustment: Angle::ZERO,
            windage_adjustment: Angle::ZERO,
            count: 0,
        }
    }
}

impl<D> Simulation<D>
where
    D: DragFunction,
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
                |p| p.distance() >= distance,
                |p| -p.offset_vertical_angle(elevation_offset),
                |p| -p.offset_horizontal_angle(windage_offset),
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
                err => Some(err),
            })
            .unwrap()?; // The iterator always returns Some - unwrap to inner result, then handle with "?"
        Ok((pitch, yaw))
    }
}
