use of::OrderedFloat;

use std::collections::BTreeMap;

pub use self::DragTableKind::*;
custom_derive! {
    #[derive(EnumFromStr)]
    pub enum DragTableKind {
        G1,
        G2,
        G5,
        G6,
        G7,
        G8,
        GI,
    }
}

pub struct DragTable(pub BTreeMap<OrderedFloat<f64>, f64>);

impl DragTable {
    fn mut_iter_insert(&mut self, mach_cd_values: Vec<(f64, f64)>) {
        for (x, y) in mach_cd_values.iter() {
            self.0.insert(OrderedFloat(*x), *y);
        }
    }
    pub fn lerp(&self, mach: f64) -> f64 {
        let key = OrderedFloat(mach);
        let (x0, y0) = self.0.range(..key).next_back().unwrap();
        let (x1, y1) = self.0.range(key..).next().unwrap();
        let (x, y) = ((x0.0, x1.0), (*y0, *y1));
        y.0 + (mach - x.0) * (y.1 - y.0) / (x.1 - x.0)
    }
    pub fn new(drag_table_kind: DragTableKind) -> Self {
        let mut drag_table = DragTable(BTreeMap::new());
        match drag_table_kind {
            G1 => {
                drag_table.mut_iter_insert(vec![
                    (0.00, 0.2629),
                    (0.05, 0.2558),
                    (0.10, 0.2487),
                    (0.15, 0.2413),
                    (0.20, 0.2344),
                    (0.25, 0.2278),
                    (0.30, 0.2214),
                    (0.35, 0.2155),
                    (0.40, 0.2104),
                    (0.45, 0.2061),
                    (0.50, 0.2032),
                    (0.55, 0.2020),
                    (0.60, 0.2034),
                    (0.70, 0.2165),
                    (0.725, 0.2230),
                    (0.75, 0.2313),
                    (0.775, 0.2417),
                    (0.80, 0.2546),
                    (0.825, 0.2706),
                    (0.85, 0.2901),
                    (0.875, 0.3136),
                    (0.90, 0.3415),
                    (0.925, 0.3734),
                    (0.95, 0.4084),
                    (0.975, 0.4448),
                    (1.0, 0.4805),
                    (1.025, 0.5136),
                    (1.05, 0.5427),
                    (1.075, 0.5677),
                    (1.10, 0.5883),
                    (1.125, 0.6053),
                    (1.15, 0.6191),
                    (1.20, 0.6393),
                    (1.25, 0.6518),
                    (1.30, 0.6589),
                    (1.35, 0.6621),
                    (1.40, 0.6625),
                    (1.45, 0.6607),
                    (1.50, 0.6573),
                    (1.55, 0.6528),
                    (1.60, 0.6474),
                    (1.65, 0.6413),
                    (1.70, 0.6347),
                    (1.75, 0.6280),
                    (1.80, 0.6210),
                    (1.85, 0.6141),
                    (1.90, 0.6072),
                    (1.95, 0.6003),
                    (2.00, 0.5934),
                    (2.05, 0.5867),
                    (2.10, 0.5804),
                    (2.15, 0.5743),
                    (2.20, 0.5685),
                    (2.25, 0.5630),
                    (2.30, 0.5577),
                    (2.35, 0.5527),
                    (2.40, 0.5481),
                    (2.45, 0.5438),
                    (2.50, 0.5397),
                    (2.60, 0.5325),
                    (2.70, 0.5264),
                    (2.80, 0.5211),
                    (2.90, 0.5168),
                    (3.00, 0.5133),
                    (3.10, 0.5105),
                    (3.20, 0.5084),
                    (3.30, 0.5067),
                    (3.40, 0.5054),
                    (3.50, 0.5040),
                    (3.60, 0.5030),
                    (3.70, 0.5022),
                    (3.80, 0.5016),
                    (3.90, 0.5010),
                    (4.00, 0.5006),
                    (4.20, 0.4998),
                    (4.40, 0.4995),
                    (4.60, 0.4992),
                    (4.80, 0.4990),
                    (5.00, 0.4988),
                ]);
            }
            G2 => {
                drag_table.mut_iter_insert(vec![
                    (0.00, 0.2303),
                    (0.05, 0.2298),
                    (0.10, 0.2287),
                    (0.15, 0.2271),
                    (0.20, 0.2251),
                    (0.25, 0.2227),
                    (0.30, 0.2196),
                    (0.35, 0.2156),
                    (0.40, 0.2107),
                    (0.45, 0.2048),
                    (0.50, 0.1980),
                    (0.55, 0.1905),
                    (0.60, 0.1828),
                    (0.65, 0.1758),
                    (0.70, 0.1702),
                    (0.75, 0.1669),
                    (0.775, 0.1664),
                    (0.80, 0.1667),
                    (0.825, 0.1682),
                    (0.85, 0.1711),
                    (0.875, 0.1761),
                    (0.90, 0.1831),
                    (0.925, 0.2004),
                    (0.95, 0.2589),
                    (0.975, 0.3492),
                    (1.0, 0.3983),
                    (1.025, 0.4075),
                    (1.05, 0.4103),
                    (1.075, 0.4114),
                    (1.10, 0.4106),
                    (1.125, 0.4089),
                    (1.15, 0.4068),
                    (1.175, 0.4046),
                    (1.20, 0.4021),
                    (1.25, 0.3966),
                    (1.30, 0.3904),
                    (1.35, 0.3835),
                    (1.40, 0.3759),
                    (1.45, 0.3678),
                    (1.50, 0.3594),
                    (1.55, 0.3512),
                    (1.60, 0.3432),
                    (1.65, 0.3356),
                    (1.70, 0.3282),
                    (1.75, 0.3213),
                    (1.80, 0.3149),
                    (1.85, 0.3089),
                    (1.90, 0.3033),
                    (1.95, 0.2982),
                    (2.00, 0.2933),
                    (2.05, 0.2889),
                    (2.10, 0.2846),
                    (2.15, 0.2806),
                    (2.20, 0.2768),
                    (2.25, 0.2731),
                    (2.30, 0.2696),
                    (2.35, 0.2663),
                    (2.40, 0.2632),
                    (2.45, 0.2602),
                    (2.50, 0.2572),
                    (2.55, 0.2543),
                    (2.60, 0.2515),
                    (2.65, 0.2487),
                    (2.70, 0.2460),
                    (2.75, 0.2433),
                    (2.80, 0.2408),
                    (2.85, 0.2382),
                    (2.90, 0.2357),
                    (2.95, 0.2333),
                    (3.00, 0.2309),
                    (3.10, 0.2262),
                    (3.20, 0.2217),
                    (3.30, 0.2173),
                    (3.40, 0.2132),
                    (3.50, 0.2091),
                    (3.60, 0.2052),
                    (3.70, 0.2014),
                    (3.80, 0.1978),
                    (3.90, 0.1944),
                    (4.00, 0.1912),
                    (4.20, 0.1851),
                    (4.40, 0.1794),
                    (4.60, 0.1741),
                    (4.80, 0.1693),
                    (5.00, 0.1648),
                ]);
            }
            G5 => {
                drag_table.mut_iter_insert(vec![
                    (0.00, 0.1710),
                    (0.05, 0.1719),
                    (0.10, 0.1727),
                    (0.15, 0.1732),
                    (0.20, 0.1734),
                    (0.25, 0.1730),
                    (0.30, 0.1718),
                    (0.35, 0.1696),
                    (0.40, 0.1668),
                    (0.45, 0.1637),
                    (0.50, 0.1603),
                    (0.55, 0.1566),
                    (0.60, 0.1529),
                    (0.65, 0.1497),
                    (0.70, 0.1473),
                    (0.75, 0.1463),
                    (0.80, 0.1489),
                    (0.85, 0.1583),
                    (0.875, 0.1672),
                    (0.90, 0.1815),
                    (0.925, 0.2051),
                    (0.95, 0.2413),
                    (0.975, 0.2884),
                    (1.0, 0.3379),
                    (1.025, 0.3785),
                    (1.05, 0.4032),
                    (1.075, 0.4147),
                    (1.10, 0.4201),
                    (1.15, 0.4278),
                    (1.20, 0.4338),
                    (1.25, 0.4373),
                    (1.30, 0.4392),
                    (1.35, 0.4403),
                    (1.40, 0.4406),
                    (1.45, 0.4401),
                    (1.50, 0.4386),
                    (1.55, 0.4362),
                    (1.60, 0.4328),
                    (1.65, 0.4286),
                    (1.70, 0.4237),
                    (1.75, 0.4182),
                    (1.80, 0.4121),
                    (1.85, 0.4057),
                    (1.90, 0.3991),
                    (1.95, 0.3926),
                    (2.00, 0.3861),
                    (2.05, 0.3800),
                    (2.10, 0.3741),
                    (2.15, 0.3684),
                    (2.20, 0.3630),
                    (2.25, 0.3578),
                    (2.30, 0.3529),
                    (2.35, 0.3481),
                    (2.40, 0.3435),
                    (2.45, 0.3391),
                    (2.50, 0.3349),
                    (2.60, 0.3269),
                    (2.70, 0.3194),
                    (2.80, 0.3125),
                    (2.90, 0.3060),
                    (3.00, 0.2999),
                    (3.10, 0.2942),
                    (3.20, 0.2889),
                    (3.30, 0.2838),
                    (3.40, 0.2790),
                    (3.50, 0.2745),
                    (3.60, 0.2703),
                    (3.70, 0.2662),
                    (3.80, 0.2624),
                    (3.90, 0.2588),
                    (4.00, 0.2553),
                    (4.20, 0.2488),
                    (4.40, 0.2429),
                    (4.60, 0.2376),
                    (4.80, 0.2326),
                    (5.00, 0.2280),
                ]);
            }
            G6 => {
                drag_table.mut_iter_insert(vec![
                    (0.00, 0.2617),
                    (0.05, 0.2553),
                    (0.10, 0.2491),
                    (0.15, 0.2432),
                    (0.20, 0.2376),
                    (0.25, 0.2324),
                    (0.30, 0.2278),
                    (0.35, 0.2238),
                    (0.40, 0.2205),
                    (0.45, 0.2177),
                    (0.50, 0.2155),
                    (0.55, 0.2138),
                    (0.60, 0.2126),
                    (0.65, 0.2121),
                    (0.70, 0.2122),
                    (0.75, 0.2132),
                    (0.80, 0.2154),
                    (0.85, 0.2194),
                    (0.875, 0.2229),
                    (0.90, 0.2297),
                    (0.925, 0.2449),
                    (0.95, 0.2732),
                    (0.975, 0.3141),
                    (1.0, 0.3597),
                    (1.025, 0.3994),
                    (1.05, 0.4261),
                    (1.075, 0.4402),
                    (1.10, 0.4465),
                    (1.125, 0.4490),
                    (1.15, 0.4497),
                    (1.175, 0.4494),
                    (1.20, 0.4482),
                    (1.225, 0.4464),
                    (1.25, 0.4441),
                    (1.30, 0.4390),
                    (1.35, 0.4336),
                    (1.40, 0.4279),
                    (1.45, 0.4221),
                    (1.50, 0.4162),
                    (1.55, 0.4102),
                    (1.60, 0.4042),
                    (1.65, 0.3981),
                    (1.70, 0.3919),
                    (1.75, 0.3855),
                    (1.80, 0.3788),
                    (1.85, 0.3721),
                    (1.90, 0.3652),
                    (1.95, 0.3583),
                    (2.00, 0.3515),
                    (2.05, 0.3447),
                    (2.10, 0.3381),
                    (2.15, 0.3314),
                    (2.20, 0.3249),
                    (2.25, 0.3185),
                    (2.30, 0.3122),
                    (2.35, 0.3060),
                    (2.40, 0.3000),
                    (2.45, 0.2941),
                    (2.50, 0.2883),
                    (2.60, 0.2772),
                    (2.70, 0.2668),
                    (2.80, 0.2574),
                    (2.90, 0.2487),
                    (3.00, 0.2407),
                    (3.10, 0.2333),
                    (3.20, 0.2265),
                    (3.30, 0.2202),
                    (3.40, 0.2144),
                    (3.50, 0.2089),
                    (3.60, 0.2039),
                    (3.70, 0.1991),
                    (3.80, 0.1947),
                    (3.90, 0.1905),
                    (4.00, 0.1866),
                    (4.20, 0.1794),
                    (4.40, 0.1730),
                    (4.60, 0.1673),
                    (4.80, 0.1621),
                    (5.00, 0.1574),
                ]);
            }
            G7 => {
                drag_table.mut_iter_insert(vec![
                    (0.00, 0.1198),
                    (0.05, 0.1197),
                    (0.10, 0.1196),
                    (0.15, 0.1194),
                    (0.20, 0.1193),
                    (0.25, 0.1194),
                    (0.30, 0.1194),
                    (0.35, 0.1194),
                    (0.40, 0.1193),
                    (0.45, 0.1193),
                    (0.50, 0.1194),
                    (0.55, 0.1193),
                    (0.60, 0.1194),
                    (0.65, 0.1197),
                    (0.70, 0.1202),
                    (0.725, 0.1207),
                    (0.754, 0.1215),
                    (0.7754, 0.1226),
                    (0.804, 0.1242),
                    (0.8254, 0.1266),
                    (0.854, 0.1306),
                    (0.8754, 0.1368),
                    (0.904, 0.1464),
                    (0.9254, 0.1660),
                    (0.954, 0.2054),
                    (0.9754, 0.2993),
                    (1.04, 0.3803),
                    (1.0254, 0.4015),
                    (1.054, 0.4043),
                    (1.0754, 0.4034),
                    (1.104, 0.4014),
                    (1.1254, 0.3987),
                    (1.15, 0.3955),
                    (1.20, 0.3884),
                    (1.25, 0.3810),
                    (1.30, 0.3732),
                    (1.35, 0.3657),
                    (1.40, 0.3580),
                    (1.50, 0.3440),
                    (1.55, 0.3376),
                    (1.60, 0.3315),
                    (1.65, 0.3260),
                    (1.70, 0.3209),
                    (1.75, 0.3160),
                    (1.80, 0.3117),
                    (1.85, 0.3078),
                    (1.90, 0.3042),
                    (1.95, 0.3010),
                    (2.00, 0.2980),
                    (2.05, 0.2951),
                    (2.10, 0.2922),
                    (2.15, 0.2892),
                    (2.20, 0.2864),
                    (2.25, 0.2835),
                    (2.30, 0.2807),
                    (2.35, 0.2779),
                    (2.40, 0.2752),
                    (2.45, 0.2725),
                    (2.50, 0.2697),
                    (2.55, 0.2670),
                    (2.60, 0.2643),
                    (2.65, 0.2615),
                    (2.70, 0.2588),
                    (2.75, 0.2561),
                    (2.80, 0.2533),
                    (2.85, 0.2506),
                    (2.90, 0.2479),
                    (2.95, 0.2451),
                    (3.00, 0.2424),
                    (3.10, 0.2368),
                    (3.20, 0.2313),
                    (3.30, 0.2258),
                    (3.40, 0.2205),
                    (3.50, 0.2154),
                    (3.60, 0.2106),
                    (3.70, 0.2060),
                    (3.80, 0.2017),
                    (3.90, 0.1975),
                    (4.00, 0.1935),
                    (4.20, 0.1861),
                    (4.40, 0.1793),
                    (4.60, 0.1730),
                    (4.80, 0.1672),
                    (5.00, 0.1618),
                ]);
            }
            G8 => {
                drag_table.mut_iter_insert(vec![
                    (0.00, 0.2105),
                    (0.05, 0.2105),
                    (0.10, 0.2104),
                    (0.15, 0.2104),
                    (0.20, 0.2103),
                    (0.25, 0.2103),
                    (0.30, 0.2103),
                    (0.35, 0.2103),
                    (0.40, 0.2103),
                    (0.45, 0.2102),
                    (0.50, 0.2102),
                    (0.55, 0.2102),
                    (0.60, 0.2102),
                    (0.65, 0.2102),
                    (0.70, 0.2103),
                    (0.75, 0.2103),
                    (0.80, 0.2104),
                    (0.825, 0.2104),
                    (0.85, 0.2105),
                    (0.875, 0.2106),
                    (0.90, 0.2109),
                    (0.925, 0.2183),
                    (0.95, 0.2571),
                    (0.975, 0.3358),
                    (1.0, 0.4068),
                    (1.025, 0.4378),
                    (1.05, 0.4476),
                    (1.075, 0.4493),
                    (1.10, 0.4477),
                    (1.125, 0.4450),
                    (1.15, 0.4419),
                    (1.20, 0.4353),
                    (1.25, 0.4283),
                    (1.30, 0.4208),
                    (1.35, 0.4133),
                    (1.40, 0.4059),
                    (1.45, 0.3986),
                    (1.50, 0.3915),
                    (1.55, 0.3845),
                    (1.60, 0.3777),
                    (1.65, 0.3710),
                    (1.70, 0.3645),
                    (1.75, 0.3581),
                    (1.80, 0.3519),
                    (1.85, 0.3458),
                    (1.90, 0.3400),
                    (1.95, 0.3343),
                    (2.00, 0.3288),
                    (2.05, 0.3234),
                    (2.10, 0.3182),
                    (2.15, 0.3131),
                    (2.20, 0.3081),
                    (2.25, 0.3032),
                    (2.30, 0.2983),
                    (2.35, 0.2937),
                    (2.40, 0.2891),
                    (2.45, 0.2845),
                    (2.50, 0.2802),
                    (2.60, 0.2720),
                    (2.70, 0.2642),
                    (2.80, 0.2569),
                    (2.90, 0.2499),
                    (3.00, 0.2432),
                    (3.10, 0.2368),
                    (3.20, 0.2308),
                    (3.30, 0.2251),
                    (3.40, 0.2197),
                    (3.50, 0.2147),
                    (3.60, 0.2101),
                    (3.70, 0.2058),
                    (3.80, 0.2019),
                    (3.90, 0.1983),
                    (4.00, 0.1950),
                    (4.20, 0.1890),
                    (4.40, 0.1837),
                    (4.60, 0.1791),
                    (4.80, 0.1750),
                    (5.00, 0.1713),
                ]);
            }
            GI => {
                drag_table.mut_iter_insert(vec![
                    (0.00, 0.2282),
                    (0.05, 0.2282),
                    (0.10, 0.2282),
                    (0.15, 0.2282),
                    (0.20, 0.2282),
                    (0.25, 0.2282),
                    (0.30, 0.2282),
                    (0.35, 0.2282),
                    (0.40, 0.2282),
                    (0.45, 0.2282),
                    (0.50, 0.2282),
                    (0.55, 0.2282),
                    (0.60, 0.2282),
                    (0.65, 0.2282),
                    (0.70, 0.2282),
                    (0.725, 0.2353),
                    (0.75, 0.2434),
                    (0.775, 0.2515),
                    (0.80, 0.2596),
                    (0.825, 0.2677),
                    (0.85, 0.2759),
                    (0.875, 0.2913),
                    (0.90, 0.3170),
                    (0.925, 0.3442),
                    (0.95, 0.3728),
                    (1.0, 0.4349),
                    (1.05, 0.5034),
                    (1.075, 0.5402),
                    (1.10, 0.5756),
                    (1.125, 0.5887),
                    (1.15, 0.6018),
                    (1.175, 0.6149),
                    (1.20, 0.6279),
                    (1.225, 0.6418),
                    (1.25, 0.6423),
                    (1.30, 0.6423),
                    (1.35, 0.6423),
                    (1.40, 0.6423),
                    (1.45, 0.6423),
                    (1.50, 0.6423),
                    (1.55, 0.6423),
                    (1.60, 0.6423),
                    (1.625, 0.6407),
                    (1.65, 0.6378),
                    (1.70, 0.6321),
                    (1.75, 0.6266),
                    (1.80, 0.6213),
                    (1.85, 0.6163),
                    (1.90, 0.6113),
                    (1.95, 0.6066),
                    (2.00, 0.6020),
                    (2.05, 0.5976),
                    (2.10, 0.5933),
                    (2.15, 0.5891),
                    (2.20, 0.5850),
                    (2.25, 0.5811),
                    (2.30, 0.5773),
                    (2.35, 0.5733),
                    (2.40, 0.5679),
                    (2.45, 0.5626),
                    (2.50, 0.5576),
                    (2.60, 0.5478),
                    (2.70, 0.5386),
                    (2.80, 0.5298),
                    (2.90, 0.5215),
                    (3.00, 0.5136),
                    (3.10, 0.5061),
                    (3.20, 0.4989),
                    (3.30, 0.4921),
                    (3.40, 0.4855),
                    (3.50, 0.4792),
                    (3.60, 0.4732),
                    (3.70, 0.4674),
                    (3.80, 0.4618),
                    (3.90, 0.4564),
                    (4.00, 0.4513),
                    (4.20, 0.4415),
                    (4.40, 0.4323),
                    (4.60, 0.4238),
                    (4.80, 0.4157),
                    (5.00, 0.4082),
                ]);
            }
        }
        drag_table
    }
}
