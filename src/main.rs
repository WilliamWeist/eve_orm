use std::time::Instant;

fn main() {
    let start: Instant = Instant::now();
    eve_orm::update_database();
    let duration = start.elapsed();
    println!("DB update time: {:.2}s", duration.as_secs_f64());
}
