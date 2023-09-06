use std::collections::HashMap;  // Import the HashMap data structure for storing todo items.
use std::error::Error;          // Import the Error trait for error handling.
use std::fs::{File, OpenOptions}; // Import file-related functions and types.

// Define a struct called Todo to represent the todo list.
struct Todo {
    map: HashMap<String, bool>, // Store todo items as key-value pairs (item name, completion status).
}

impl Todo {
    // Constructor for Todo.
    fn new() -> Result<Todo, Box<dyn Error>> {
        // Open or create a file with read and write permissions for storing todo data.
        let f = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(Self::get_filename())?; // Use ? to handle potential errors.

        // Deserialize todo data from the file, or create a new HashMap if the file is empty.
        let map: HashMap<String, bool> = match serde_json::from_reader(f) {
            Ok(map) => map,
            Err(e) if e.is_eof() => HashMap::new(),
            Err(e) => return Err(e.into()), // Convert the error into a Box<dyn Error>.
        };

        // Create a new Todo instance and return it.
        Ok(Todo { map })
    }

    // Define a function to get the filename where todo data is stored.
    fn get_filename() -> &'static str {
        "db.json"
    }

    // Mark a todo item as complete.
    fn complete(&mut self, key: &str) -> Option<()> {
        // If the item exists in the map, set its completion status to false.
        if let Some(v) = self.map.get_mut(key) {
            *v = false;
            Some(())
        } else {
            None // Return None if the item is not found.
        }
    }

    // Insert a new todo item with a given key and completion status into the map.
    fn insert(&mut self, key: String, value: bool) {
        self.map.insert(key, value);
    }

    // Save the current todo map to a file in JSON format.
    fn save(&self) -> Result<(), Box<dyn Error>> {
        // Create or overwrite the file for saving todo data.
        let f = File::create(Self::get_filename())?;

        // Serialize and write the todo map to the file in pretty-printed JSON format.
        serde_json::to_writer_pretty(f, &self.map)?;

        Ok(()) // Return Ok(()) to indicate success.
    }
}

// Entry point of the program.
fn main() -> Result<(), Box<dyn Error>> {
    // Collect command-line arguments into a vector of strings.
    let args: Vec<String> = std::env::args().collect();

    // Check if there are at least three command-line arguments.
    if args.len() < 3 {
        println!("Please specify an action and an item");
        return Ok(()); // Return early with Ok(()) if arguments are missing.
    }

    // Extract the action and item from command-line arguments.
    let action = &args[1];
    let item = &args[2];

    // Create a new Todo instance and handle potential errors.
    let mut todo = Todo::new()?;

    // Match the specified action and perform the corresponding operation.
    match action.as_str() {
        "add" => {
            todo.insert(item.to_string(), true);
            todo.save()?; // Save the updated todo list.
            println!("Item added and todo saved");
        }
        "complete" => {
            if let Some(_) = todo.complete(item) {
                todo.save()?; // Save the updated todo list.
                println!("Item marked as complete and todo saved");
            } else {
                println!("'{}' is not present in the list", item);
            }
        }
        _ => println!("Invalid action: {}", action), // Handle invalid actions.
    }

    Ok(()) // Return Ok(()) to indicate successful program execution.
}

// cargo run -- add "code rust"
// cat db.json