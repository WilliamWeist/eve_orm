use std::{collections::HashMap, time::Instant};

use eve_orm::universe::system;

fn main() {
    let start: Instant = Instant::now();
    eve_orm::update_database();
    let duration = start.elapsed();
    println!("DB update time: {:.2}s", duration.as_secs_f64());

    let systems: HashMap<i64, system::System> = system::get_all();
    println!("Number of systems: {}", &systems.len());
    let systems: HashMap<i64, system::System> = systems
        .into_iter()
        .filter(|system| {
            system.1.constellation.region.galaxy.name == "NEW EDEN"
                || system.1.constellation.region.galaxy.name == "ANOIKIS"
        })
        .collect();
    println!("Number of systems: {}", &systems.len());
    let turnur = systems.get(&30002086);
    if let Some(turnur) = turnur {
        println!("Turnur: {:?}", turnur);
    }
}
