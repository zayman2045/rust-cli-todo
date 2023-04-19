use std::{fmt, io, process};


#[derive(Debug, PartialEq)]
pub enum Status {
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
pub struct Task {
    pub name: String,
    pub status: Status,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            name,
            status: Status::Incomplete,
        }
    }
}

#[derive(Debug)]
pub struct List {
    pub items: Vec<Task>,
    pub length: i32,
}

impl List {
    pub fn new() -> Self {
        Self {
            items: vec![],
            length: 0,
        }
    }
    pub fn add_task(&mut self, task: Task) {
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


// Take user input and return a command and a name
pub fn take_command() -> (String, String) {
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
    } else if buffer.split_whitespace().count() == 1 {
        println!("Invalid Usage. Use todo --help for more information");
            process::exit(2)
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

    if !["add", "complete", "delete", "todo"].contains(&command) {
        println!("Invalid command");
        process::exit(3)
    }

    (command.to_string(), name.to_string())
}


// Create a function that takes command and name and returns a Result<Task, Err>?
// Allow this function to handle the command and operate on the List or propagate and error
