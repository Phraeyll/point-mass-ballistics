use super::{
    iter::{Output, Packet},
    *,
};

impl super::Simulation<'_> {
    // Produce a drop table using specified range and step size
    pub fn drop_table(&self, step: u32, range_start: u32, range_end: u32) -> FloatMap<Packet<'_>> {
        let mut iter = self.iter().fuse();
        (range_start..=range_end)
            .step_by(step as usize)
            .filter_map(|current_step| {
                iter.by_ref()
                    .find(|p| p.distance() >= Numeric::from(current_step))
                    .map(|p| (p.distance(), p))
            })
            .collect::<FloatMap<_>>()
    }
}
