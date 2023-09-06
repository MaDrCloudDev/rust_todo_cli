use std::collections::HashMap

fn main() {
let action: String = std::env::args().nth(1).expect("Please specify an action");
let item: String = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);
}

struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }
}

// cargo run main.rs
// cargo run -- hello world!