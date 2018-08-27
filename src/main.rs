extern crate ballistics;

use ballistics::{cdtables::*, projectiles::*};

use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() <= 13 {
        eprintln!("error: wrong number of args");
        usage(argv[0].to_string());
        return;
    }

    let initial_velocity: f64 = argv[1].parse().unwrap(); // ft/s
    let weight: f64 = argv[2].parse().unwrap(); // grains
    let caliber: f64 = argv[3].parse().unwrap(); // inches
    let bc: f64 = argv[4].parse().unwrap(); // dimensionless
    let dragtable: TableKind = argv[5].parse().unwrap(); // Desired drag table (G1, G7, etc.)
    let wind_velocity: f64 = argv[6].parse().unwrap(); // m/h
    let wind_angle: f64 = argv[7].parse().unwrap(); // degrees
    let temp: f64 = argv[8].parse().unwrap(); // F
    let pressure: f64 = argv[9].parse().unwrap(); // inHg
    let humidity: f64 = argv[10].parse().unwrap(); // dimensionless, percentage
    let range: f64 = argv[11].parse().unwrap(); // range in yd
    let step: f64 = argv[12].parse().unwrap(); // step output in yd
    let ts_factor: f64 = argv[13].parse().unwrap(); // factor to determin step size

    let mut projectile = Projectile::new(
        weight,
        caliber,
        bc,
        initial_velocity,
        wind_velocity,
        wind_angle,
        temp,
        pressure,
        humidity,
    );
    let table = Table::new(dragtable);
    let timestep: f64 = 1.0 / (ts_factor * initial_velocity);

    println!("time(s), velocity(ft/s), distance(yd), drop(in), windage(in)");
    let mut current_step: f64 = step;
    while projectile.distance() <= range {
        projectile.step_forward(&table, timestep);
        if projectile.distance() > current_step {
            println!(
                "{} {} {} {} {}",
                projectile.time(),
                projectile.velocity(),
                projectile.distance(),
                projectile.drop(),
                projectile.windage(),
            );
            current_step += step;
        }
    }
}

fn usage(name: String) {
    println!(
        r#"
        Usage: {} 
        velocity (ft/s)
        weight (gr)
        caliber (in)
        bc
        dragtable
        wind_velocity (ft/s)
        wind_angle (deg)
        temp (F)
        pressure (inHg)
        humidity (0-1)
        range (yd)
        step (yd)
        timestep_factor
        "#,
        name
    );
}
