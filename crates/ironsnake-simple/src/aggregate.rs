use std::collections::HashMap;
use std::f64::consts::PI;
pub struct Aggregate {
    pub int: i32,
    pub float_number: f64,
    pub text: String,
    pub list: Vec<f64>,
    pub tuple_data: (bool, i64),
    pub map: HashMap<String, i32>,
}

impl Aggregate {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("one".to_string(), 1);
        map.insert("two".to_string(), 2);

        Aggregate {
            int: 42,
            float_number: PI,
            text: "Hello, Python!".to_string(),
            list: vec![1.1, 2.2, 3.3],
            tuple_data: (true, 1234567890),
            map,
        }
    }
}

// https://rust-lang.github.io/rust-clippy/master/index.html#/new_without_default
impl Default for Aggregate {
    fn default() -> Self {
        Self::new()
    }
}
