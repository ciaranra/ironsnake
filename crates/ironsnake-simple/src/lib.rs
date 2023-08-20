use std::collections::HashMap;

pub fn five() -> i32 {
    5
}

pub fn create_tuple() -> (String, i32, f64) {
    ("Hello, World!".to_string(), 42, 3.14)
}

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
            float_number: 3.14,
            text: "Hello, Python!".to_string(),
            list: vec![1.1, 2.2, 3.3],
            tuple_data: (true, 1234567890),
            map,
        }
    }
}

pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

pub fn generate_person() -> Person {
    Person::new("Alice", 30)
}
