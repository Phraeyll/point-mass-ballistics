extern crate ordered_float as of;

use self::of::OrderedFloat;

pub use self::TableKind::*;

use std::collections::BTreeMap;

pub enum TableKind {
    G1,
    G2,
    G5,
    G6,
    G7,
    G8,
    GI,
}

#[derive(Debug)]
pub struct Table(pub BTreeMap<OrderedFloat<f64>, f64>);

pub trait Tabular {
    type K;
    fn insert(&mut self, f64, f64);
    fn new(Self::K) -> Self;
}

impl Tabular for Table {
    type K = TableKind;
    fn insert(&mut self, k: f64, v: f64) {
        self.0.insert(OrderedFloat(k), v);
    }
    fn new(k: Self::K) -> Self {
        let mut t = Table(BTreeMap::new());
        match k {
            G7 => {
                t.insert(0.00, 0.1198);
                t.insert(0.05, 0.1197);
                t.insert(0.10, 0.1196);
                t.insert(0.15, 0.1194);
                t.insert(0.20, 0.1193);
                t.insert(0.25, 0.1194);
                t.insert(0.30, 0.1194);
                t.insert(0.35, 0.1194);
                t.insert(0.40, 0.1193);
                t.insert(0.45, 0.1193);
                t.insert(0.50, 0.1194);
                t.insert(0.55, 0.1193);
                t.insert(0.60, 0.1194);
                t.insert(0.65, 0.1197);
                t.insert(0.70, 0.1202);
                t.insert(0.725, 0.1207);
                t.insert(0.754, 0.1215);
                t.insert(0.7754, 0.1226);
                t.insert(0.804, 0.1242);
                t.insert(0.8254, 0.1266);
                t.insert(0.854, 0.1306);
                t.insert(0.8754, 0.1368);
                t.insert(0.904, 0.1464);
                t.insert(0.9254, 0.1660);
                t.insert(0.954, 0.2054);
                t.insert(0.9754, 0.2993);
                t.insert(1.04, 0.3803);
                t.insert(1.0254, 0.4015);
                t.insert(1.054, 0.4043);
                t.insert(1.0754, 0.4034);
                t.insert(1.104, 0.4014);
                t.insert(1.1254, 0.3987);
                t.insert(1.15, 0.3955);
                t.insert(1.20, 0.3884);
                t.insert(1.25, 0.3810);
                t.insert(1.30, 0.3732);
                t.insert(1.35, 0.3657);
                t.insert(1.40, 0.3580);
                t.insert(1.50, 0.3440);
                t.insert(1.55, 0.3376);
                t.insert(1.60, 0.3315);
                t.insert(1.65, 0.3260);
                t.insert(1.70, 0.3209);
                t.insert(1.75, 0.3160);
                t.insert(1.80, 0.3117);
                t.insert(1.85, 0.3078);
                t.insert(1.90, 0.3042);
                t.insert(1.95, 0.3010);
                t.insert(2.00, 0.2980);
                t.insert(2.05, 0.2951);
                t.insert(2.10, 0.2922);
                t.insert(2.15, 0.2892);
                t.insert(2.20, 0.2864);
                t.insert(2.25, 0.2835);
                t.insert(2.30, 0.2807);
                t.insert(2.35, 0.2779);
                t.insert(2.40, 0.2752);
                t.insert(2.45, 0.2725);
                t.insert(2.50, 0.2697);
                t.insert(2.55, 0.2670);
                t.insert(2.60, 0.2643);
                t.insert(2.65, 0.2615);
                t.insert(2.70, 0.2588);
                t.insert(2.75, 0.2561);
                t.insert(2.80, 0.2533);
                t.insert(2.85, 0.2506);
                t.insert(2.90, 0.2479);
                t.insert(2.95, 0.2451);
                t.insert(3.00, 0.2424);
                t.insert(3.10, 0.2368);
                t.insert(3.20, 0.2313);
                t.insert(3.30, 0.2258);
                t.insert(3.40, 0.2205);
                t.insert(3.50, 0.2154);
                t.insert(3.60, 0.2106);
                t.insert(3.70, 0.2060);
                t.insert(3.80, 0.2017);
                t.insert(3.90, 0.1975);
                t.insert(4.00, 0.1935);
                t.insert(4.20, 0.1861);
                t.insert(4.40, 0.1793);
                t.insert(4.60, 0.1730);
                t.insert(4.80, 0.1672);
                t.insert(5.00, 0.1618);
            }
            _ => {
                t.insert(0.0, 0.0);
            }
        }
        t
    }
}
