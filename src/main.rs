extern crate ballistics;

use ballistics::{cdtables::*, consts::*, projectiles::*};

fn usage(name: String) {
    println!("Usage: {} velocity weight caliber bc range wind_velocity wind_angle temp pressure humidity", name);
}

fn main() {
    let argv: Vec<String> = ::std::env::args().collect();

    if argv.len() <= 10 {
        eprintln!("error: wrong number of args");
        usage(argv[0].to_string());
        return;
    }

    let velocity: f64 = argv[1].parse().unwrap(); // ft/s
    let weight: f64 = argv[2].parse().unwrap(); // grains
    let caliber: f64 = argv[3].parse().unwrap(); // inches
    let bc: f64 = argv[4].parse().unwrap(); // dimensionless, implied inches/lbs
    let range: f64 = argv[5].parse().unwrap(); // yards
    let wind_velocity: f64 = argv[6].parse().unwrap(); // m/h
    let wind_angle: f64 = argv[7].parse().unwrap(); // degrees
    let temp: f64 = argv[8].parse().unwrap(); // f
    let pressure: f64 = argv[9].parse().unwrap(); // inHg
    let humidity: f64 = argv[10].parse().unwrap(); // dimensionless, percentage

    let timestep: f64 = 1.0 / (4.0 * velocity);
    let con = Conditions::new(wind_velocity, wind_angle, temp, pressure, humidity);
    let table = Table::new(G7);
    // println!("{:?}", con);
    // println!("{:?}", table);

    let mut projectile = Projectile::new(weight, caliber, bc, velocity);
    while projectile.p[0] < (range * YARDS_TO_METERS) {
        projectile.step_forward(timestep, &con, &table);
    }
    println!(
        "t: {}, v: {}, x: {}, y: {}\n",
        projectile.t,
        projectile.vnorm() * METERS_TO_FEET,
        projectile.p[0] * METERS_TO_YARDS,
        projectile.p[1] * METERS_TO_INCHES,
    );
}
