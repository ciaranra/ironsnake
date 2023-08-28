pub mod person;
pub use person::*;
pub mod aggregate;
pub use aggregate::*;

use std::f64::consts::PI;

pub fn five() -> i32 {
    5
}

pub fn create_tuple() -> (String, i32, f64) {
    ("Hello, World!".to_string(), 42, PI)
}
