use of::OrderedFloat;

use std::collections::BTreeMap;

pub struct FloatMap(pub BTreeMap<OrderedFloat<f64>, f64>);

macro_rules! drag_table {
    ( $($mach:expr => $cd:expr,)+ ) => ( drag_table!($($mach => $cd),+) );
    ( $($mach:expr => $cd:expr),* ) => {{
        let mut _drag_table = FloatMap(BTreeMap::new());
        $(
            let _ = _drag_table.0.insert(OrderedFloat($mach), $cd);
        )*
        _drag_table
    }}
}
