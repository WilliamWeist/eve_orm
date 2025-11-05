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
