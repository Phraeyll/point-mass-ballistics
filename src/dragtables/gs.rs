use of::OrderedFloat;

use macros::FloatMap;
use util::Numeric;

use std::collections::BTreeMap;

pub fn init() -> FloatMap<Numeric> {
    float_map!{
        0.00 => 0.4662,
        0.05 => 0.4689,
        0.10 => 0.4717,
        0.15 => 0.4745,
        0.20 => 0.4772,
        0.25 => 0.4800,
        0.30 => 0.4827,
        0.35 => 0.4852,
        0.40 => 0.4882,
        0.45 => 0.4920,
        0.50 => 0.4970,
        0.55 => 0.5080,
        0.60 => 0.5260,
        0.65 => 0.5590,
        0.70 => 0.5920,
        0.75 => 0.6258,
        0.80 => 0.6610,
        0.85 => 0.6985,
        0.90 => 0.7370,
        0.95 => 0.7757,
        1.0 => 0.8140,
        1.05  => 0.8512,
        1.10 => 0.8870,
        1.15 => 0.9210,
        1.20 => 0.9510,
        1.25 => 0.9740,
        1.30 => 0.9910,
        1.35 => 0.9990,
        1.40 => 1.0030,
        1.45 => 1.0060,
        1.50 => 1.0080,
        1.55 => 1.0090,
        1.60 => 1.0090,
        1.65 => 1.0090,
        1.70 => 1.0090,
        1.75 => 1.0080,
        1.80 => 1.0070,
        1.85 => 1.0060,
        1.90 => 1.0040,
        1.95 => 1.0025,
        2.00 => 1.0010,
        2.05 => 0.9990,
        2.10 => 0.9970,
        2.15 => 0.9956,
        2.20 => 0.9940,
        2.25 => 0.9916,
        2.30 => 0.9890,
        2.35 => 0.9869,
        2.40 => 0.9850,
        2.45 => 0.9830,
        2.50 => 0.9810,
        2.55 => 0.9790,
        2.60 => 0.9770,
        2.65 => 0.9750,
        2.70 => 0.9730,
        2.75 => 0.9710,
        2.80 => 0.9690,
        2.85 => 0.9670,
        2.90 => 0.9650,
        2.95 => 0.9630,
        3.00 => 0.9610,
        3.05 => 0.9589,
        3.10 => 0.9570,
        3.15 => 0.9555,
        3.20 => 0.9540,
        3.25 => 0.9520,
        3.30 => 0.9500,
        3.35 => 0.9485,
        3.40 => 0.9470,
        3.45 => 0.9450,
        3.50 => 0.9430,
        3.55 => 0.9414,
        3.60 => 0.9400,
        3.65 => 0.9385,
        3.70 => 0.9370,
        3.75 => 0.9355,
        3.80 => 0.9340,
        3.85 => 0.9325,
        3.90 => 0.9310,
        3.95 => 0.9295,
        4.00 => 0.9280,
    }
}
