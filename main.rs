// TODO:
// Consider using hashmap to store tasks. Maybe in an alternate version
// Allow user to add, complete and delete Tasks, as well as displaying their status
// Require format: (add, complete, delete) (task name)
// Add --help configuration for help/usage, --display for displaying List
// Implement Display for List. Include the items and length of List
// Move bulk of project into library crate
// Add documentation comments
use std::{fmt, io, process};

#[derive(Debug, PartialEq)]
enum Status {
    Incomplete,
    Complete,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Complete => write!(f, "{}", "Complete"),
            Status::Incomplete => write!(f, "{}", "Incomplete")
        }
    }
}

#[derive(Debug, PartialEq)]
struct Task {
    name: String,
    status: Status,
}

impl Task {
    fn new(name: String) -> Self {
        Self {
            name,
            status: Status::Incomplete,
        }
    }
}

// impl PartialEq for Task {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name && self.status == other.status
//     }
// }

#[derive(Debug)]
struct List {
    items: Vec<Task>,
    length: i32,
}

impl List {
    fn new() -> Self {
        Self {
            items: vec![],
            length: 0,
        }
    }
    fn add_task(&mut self, task: Task) {
        self.items.push(task);
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nTodo: {} current items.\n-----\n", self.length)?;
        for i in 0..self.items.len() {
            write!(f, "{}, {}\n", self.items[i].name, self.items[i].status)?;
        }
        write!(f, "\nEnter command:")
    }
}

fn main() {
    // Create List
    let mut list = List::new();
    println!("\nTodo: No current items.\n-----\n\nEnter command:");

    loop {
        // Obtain the command
        let (command, name) = take_command();

        // Exit program
        if command == "exit" {
            println!("Exiting program...");
            break;
        }

        // Add new task if it does not already exist
        if command == "add" && !list.items.iter().any(|task| task.name == name && (task.status == Status::Complete || task.status == Status::Incomplete) ){
            let task = Task::new(name);
            list.add_task(task);
        }

        // 

        println!("{}", list);
    }

    // TODO:
    // Only add item if it does not already exist in List
    // Only complete item if it has been added
    // Can delete item if it has been completed or deleted

    ();
}

// Take user input and return a command and a name
fn take_command() -> (String, String) {
    // Take user input as string
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => (),
        Err(_) => {
            println!("Error obtaining input");
            process::exit(1)
        }
    }

    if buffer == "exit\n" {
        return ("exit".to_string(), String::new());
    }

    // Parse buffer into command and content
    let (command, name) = match buffer.split_once(" ") {
        Some(input) => (input.0, input.1.trim()),
        None => {
            println!("Error parsing input");
            process::exit(2)
        }
    };

    let (command, name) = (command.trim(), name.trim());

    if !["add", "complete", "delete"].contains(&command) {
        println!("Invalid command");
        process::exit(3)
    }

    (command.to_string(), name.to_string())
}

// Take command and name and return a Result<Task, Err>?
// if the name exits already and the 
