use std::collections::HashMap;
use std::io::{Error, Write};

fn main() {
    let action: String = std::env::args().nth(1).expect("Please specify an action");
    let item: String = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new();

    if action == "add" {
        todo.insert(item.clone(), true); // Insert an item with a boolean value.
        match todo.save() {
            Ok(_) => println!("Todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }

    println!("{:?}, {:?}", action, item);
}

struct Todo {
    // Use Rust's built-in HashMap to store key-value pairs.
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Todo {
        Todo {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: bool) {
        self.map.insert(key, value);
    }

    fn save(&self) -> Result<(), Error> {
        let mut file = std::fs::File::create("db.txt")?;

        for (k, v) in &self.map {
            let record = format!("{}\t{}\n", k, v);
            file.write_all(record.as_bytes())?;
        }

        Ok(())
    }
}

// cargo run -- add "code rust"
// cat db.txt