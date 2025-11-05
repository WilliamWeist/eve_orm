use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use eve_orm::universe::system;

fn main() {
    let start: Instant = Instant::now();
    eve_orm::update_database();
    let duration = start.elapsed();
    println!("DB update time: {:.2}s", duration.as_secs_f64());

    let start: Instant = Instant::now();
    let systems_cache: HashMap<u64, system::System> = system::get_all();
    let duration: Duration = start.elapsed();
    println!("Creating systems cache: {:.2}s", duration.as_secs_f64());

    let query: &str = "hek";
    let start: Instant = Instant::now();
    let systems: Vec<system::System> = system::search(query, Some(&systems_cache));
    let duration: Duration = start.elapsed();
    println!(
        "Found {} systems in: {:.2}s for query: {}",
        systems.len(),
        duration.as_secs_f64(),
        query
    );

    let query: &str = "aMamaa";
    let start: Instant = Instant::now();
    let systems: Vec<system::System> = system::search(query, Some(&systems_cache));
    let duration: Duration = start.elapsed();
    println!(
        "Found {} systems in: {:.2}s for query: {}",
        systems.len(),
        duration.as_secs_f64(),
        query
    );
}
