use of::OrderedFloat;

use std::collections::BTreeMap;

pub struct FloatMap<T>(pub BTreeMap<OrderedFloat<f64>, T>);
impl<T> FloatMap<T> {
    pub fn default() -> Self {
        FloatMap(
            BTreeMap::new()
        )
    }
}

macro_rules! float_map {
    ( $($mach:expr => $cd:expr,)+ ) => ( float_map!($($mach => $cd),+) );
    ( $($mach:expr => $cd:expr),* ) => {{
        let mut _float_map = FloatMap(BTreeMap::new());
        $(
            let _ = _float_map.0.insert(OrderedFloat($mach), $cd);
        )*
        _float_map
    }}
}
