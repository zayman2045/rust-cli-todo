use std::{fmt, io, process};

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Incomplete,
    Complete,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Complete => write!(f, "{}", "Complete"),
            Status::Incomplete => write!(f, "{}", "Incomplete"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    pub completed_tasks: i32,
    pub incomplete_tasks: i32,
}

impl List {
    pub fn new() -> Self {
        Self {
            items: vec![],
            completed_tasks: 0,
            incomplete_tasks: 0,
        }
    }

    // Calculate the number of completed and incomplete tasks in a List
    pub fn calculate_items(&mut self) {
        let mut completed = 0;
        let mut incomplete = 0;
        self.items.iter().for_each(|item| {
            if item.status == Status::Complete {
                completed += 1
            } else {
                incomplete += 1
            }
        });

        self.completed_tasks = completed;
        self.incomplete_tasks = incomplete;
    }

    pub fn add_task(&mut self, task: Task) {
        self.items.push(task);
        self.calculate_items()
    }

    pub fn complete_task(&mut self, input_name: &String) {
        self.items = self
            .items
            .iter()
            .map(|task| {
                if &task.name == input_name {
                    Task {
                        name: input_name.clone(),
                        status: Status::Complete,
                    }
                } else {
                    Task {
                        name: task.name.clone(),
                        status: task.status.clone(),
                    }
                }
            })
            .collect();
        self.calculate_items();
    }

    pub fn delete_task(&mut self, input_name: &String) {
        self.items = self.items.iter().filter(|task| task.name != *input_name).cloned().collect();
        self.calculate_items();
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let title = "TODO LIST";
        let bar = "|";
        let task_count = format!("{} completed tasks. {} incomplete tasks.", self.completed_tasks, self.incomplete_tasks);
        write!(
            f,
            "\n{:-^50}\n|{task_count: ^48}|\n{bar: <25}{bar: >25}\n",
            title
        )?;
        for i in 0..self.items.len() {
            let task_output = format!("{}: {}", self.items[i].name, self.items[i].status);
            write!(f, "|{task_output: ^48}|\n")?;
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
        Some(input) => (input.0, input.1),
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_added() {
        let mut list = List::new();
        let test_task = Task::new("test".to_string());
        list.add_task(test_task);
        assert_eq!(list.incomplete_tasks, 1);
        assert_eq!(list.completed_tasks, 0);
    }

    #[test]
    fn task_completed() {
        let mut list = List::new();
        let test_task = Task::new("test".to_string());
        list.add_task(test_task);
        list.complete_task(&"test".to_string());
        assert_eq!(list.incomplete_tasks, 0);
        assert_eq!(list.completed_tasks, 1);
    }

    #[test]
    fn incomplete_task_deleted() {
        let mut list = List::new();
        let test_task = Task::new("test".to_string());
        list.add_task(test_task);
        assert_eq!(list.incomplete_tasks, 1);
        list.delete_task(&"test".to_string());
        assert_eq!(list.incomplete_tasks, 0);
    }

    #[test]
    fn completed_task_deleted() {
        let mut list = List::new();
        let test_task = Task::new("test".to_string());
        list.add_task(test_task);
        list.complete_task(&"test".to_string());
        assert_eq!(list.completed_tasks, 1);
        assert_eq!(list.incomplete_tasks, 0);
        list.delete_task(&"test".to_string());
        assert_eq!(list.completed_tasks, 0);
        assert_eq!(list.incomplete_tasks, 0);
    }
}