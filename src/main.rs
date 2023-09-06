use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Please specify an action and an item");
        return Ok(());
    }

    let action = &args[1];
    let item = &args[2];

    let mut todo = Todo::new()?;

    match action.as_str() {
        "add" => {
            todo.insert(item.to_string(), true);
            todo.save()?;
            println!("Item added and todo saved");
        }
        "complete" => {
            if let Some(_) = todo.complete(item) {
                todo.save()?;
                println!("Item marked as complete and todo saved");
            } else {
                println!("'{}' is not present in the list", item);
            }
        }
        _ => println!("Invalid action: {}", action),
    }

    Ok(())
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, Box<dyn Error>> {
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        let map: HashMap<String, bool> = match serde_json::from_reader(f) {
            Ok(map) => map,
            Err(e) if e.is_eof() => HashMap::new(),
            Err(e) => return Err(e.into()),
        };

        Ok(Todo { map })
    }

    fn complete(&mut self, key: &str) -> Option<()> {
        if let Some(v) = self.map.get_mut(key) {
            *v = false;
            Some(())
        } else {
            None
        }
    }

    fn insert(&mut self, key: String, value: bool) {
        self.map.insert(key, value);
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let f = File::create("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }
}

// cargo run -- add "code rust"
// cat db.json