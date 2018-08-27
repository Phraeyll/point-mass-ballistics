extern crate ballistics;

use ballistics::{cdtables::*, consts::*, projectiles::*};

fn usage(name: String) {
    println!("Usage: {} velocity:ft/s weight:gr caliber:in bc wind_velocity:ft/s wind_angle temp:F pressure:inHg humidity:0-1", name);
}

fn main() {
    let argv: Vec<String> = ::std::env::args().collect();

    if argv.len() <= 9 {
        eprintln!("error: wrong number of args");
        usage(argv[0].to_string());
        return;
    }

    let velocity: f64 = argv[1].parse().unwrap(); // ft/s
    let weight: f64 = argv[2].parse().unwrap(); // grains
    let caliber: f64 = argv[3].parse().unwrap(); // inches
    let bc: f64 = argv[4].parse().unwrap(); // dimensionless, implied inches/lbs
    let wind_velocity: f64 = argv[5].parse().unwrap(); // m/h
    let wind_angle: f64 = argv[6].parse().unwrap(); // degrees
    let temp: f64 = argv[7].parse().unwrap(); // f
    let pressure: f64 = argv[8].parse().unwrap(); // inHg
    let humidity: f64 = argv[9].parse().unwrap(); // dimensionless, percentage

    let timestep: f64 = 1.0 / (4.0 * velocity);
    let con = Conditions::new(wind_velocity, wind_angle, temp, pressure, humidity);
    let table = Table::new(G7);
    // println!("{:?}", con);
    // println!("{:?}", table);

    let mut projectile = Projectile::new(weight, caliber, bc, velocity);
    let printouts = [0.0f64, 100.0f64, 200.0f64, 300.0f64, 400.0f64, 500.0f64, 600.0f64, 700.0f64, 800.0f64, 900.0f64, 1000.0f64];
    let mut start: usize = 0;
    let range: f64 = 1000.0f64;
    while projectile.p[0] <= (range * YARDS_TO_METERS) {
        projectile.step_forward(timestep, &con, &table);
        if (projectile.p[0] * METERS_TO_YARDS) > printouts[start] {
                println!(
                    "t: {}, v: {}, x: {}, y: {}\n",
                    projectile.t,
                    projectile.vnorm() * METERS_TO_FEET,
                    projectile.p[0] * METERS_TO_YARDS,
                    projectile.p[1] * METERS_TO_INCHES,
                );
                if start < printouts.len() {
                    start += 1;
                }
        }
    }
}
