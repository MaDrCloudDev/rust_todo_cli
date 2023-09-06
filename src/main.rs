use std::collections::HashMap;
use std::io::{Error, Read, Write};
use std::str::FromStr;

fn main() {
    let action: String = std::env::args().nth(1).expect("Please specify an action");
    let item: String = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new().expect("Initialisation of db failed");

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
    fn new() -> Result<Todo, std::io::Error> {
        // open the db file
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        // read its content into a new string   
        let mut content = String::new();
        f.read_to_string(&mut content)?;
    
        // allocate an empty HashMap
        let mut map = HashMap::new();
    
        // loop over each line of the file
        for entries in content.lines() {
            // split and bind values
            let mut values = entries.split('\t');
            let key = values.next().expect("No Key");
            let val = values.next().expect("No Value");
            // parse the value as a boolean with error handling
            let value = bool::from_str(val).map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Failed to parse value as boolean: {}", e),
                )
            })?;
            // insert them into HashMap
            map.insert(String::from(key), value);
        }
        // Return Ok
        Ok(Todo { map })
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