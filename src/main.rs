extern crate ballistics;

use ballistics::{cdtables::*, projectiles::*};

fn usage(name: String) {
    println!("Usage: {} velocity(ft/s) weight(gr) caliber(in) bc dragtable wind_velocity(ft/s) wind_angle(deg) temp(F) pressure(inHg) humidity(0-1) range(yd) step(yd)", name);
}

fn main() {
    let argv: Vec<String> = ::std::env::args().collect();

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
    let timestep: f64 = 1.0 / (ts_factor * initial_velocity);
    let table = Table::new(dragtable);

    let printouts: Vec<f64> = (0..((range / step) as u32 + 2))
        .into_iter()
        .map(|n| n as f64 * step)
        .collect();

    let mut start: usize = 0;
    println!("time(s), velocity(ft/s), distance(yd), drop(in), windage(in)");
    while projectile.distance() <= range {
        projectile.step_forward(&table, timestep);
        if projectile.distance() > printouts[start] {
            println!(
                "{} {} {} {} {}",
                projectile.time(),
                projectile.velocity(),
                projectile.distance(),
                projectile.drop(),
                projectile.windage(),
            );
            if start < printouts.len() {
                start += 1;
            }
        }
    }
}
