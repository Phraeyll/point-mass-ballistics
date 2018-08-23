extern crate ballistics;
extern crate ndarray;
extern crate ndarray_linalg;

use ndarray_linalg::*;

use ballistics::{consts::*, projectiles::*};

fn usage(name: String) {
    println!("Usage: {} velocity weight caliber bc range temp wind_velocity wind_angle pressure humidity cd", name);
}

fn main() {
    let argv: Vec<String> = ::std::env::args().collect();

    if argv.len() <= 11 {
        eprintln!("error: wrong number of args");
        usage(argv[0].to_string());
        return;
    }

    let velocity: f64 = argv[1].parse().unwrap(); // ft/s
    let weight: f64 = argv[2].parse().unwrap(); // grains
    let caliber: f64 = argv[3].parse().unwrap(); // inches
    let bc: f64 = argv[4].parse().unwrap(); // dimensionless, implied inches/lbs
    let range: f64 = argv[5].parse().unwrap(); // yards
    let temp: f64 = argv[6].parse().unwrap(); // f
    let wind_velocity: f64 = argv[7].parse().unwrap(); // m/h
    let wind_angle: f64 = argv[8].parse().unwrap(); // degrees
    let pressure: f64 = argv[9].parse().unwrap(); // inHg
    let humidity: f64 = argv[10].parse().unwrap(); // dimensionless, percentage
    let cd: f64 = argv[11].parse().unwrap(); // coefficient of drag - testing

    let timestep: f64 = 1.0 / (4.0 * velocity);
    let c = Conditions::new(temp, wind_velocity, wind_angle, pressure, humidity);
    println!("{:?}", c);

    let mut projectile = Projectile::new(weight, caliber, bc, velocity);
    while projectile.p[0] < (range * YARDS_TO_METERS) {
        projectile.step_forward(timestep, &c, cd);
    }
    println!(
        "t: {}, v: {}, x: {}, y: {}",
        projectile.t,
        projectile.v.norm_l2() * METERS_TO_FEET,
        projectile.p[0] * METERS_TO_YARDS,
        projectile.p[1] * METERS_TO_INCHES,
    );
}
