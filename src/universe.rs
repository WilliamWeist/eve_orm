pub mod constellation;
pub mod galaxy;
pub mod region;
pub mod stargate;
pub mod system;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        // Remove some digit precision to reduce float error
        // ie: -8.85107925999806e+16 become -8.851079259998058e+16 when SELECT from rusqlite
        let digits = 100.0;
        (self.x / digits).round() * digits == (other.x / digits).round() * digits
            && (self.y / digits).round() * digits == (other.y / digits).round() * digits
            && (self.z / digits).round() * digits == (other.z / digits).round() * digits
    }
}
