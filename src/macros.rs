use of::OrderedFloat;

use std::collections::BTreeMap;

#[derive(Default)]
pub struct FloatMap<T>(pub BTreeMap<OrderedFloat<f64>, T>);

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
