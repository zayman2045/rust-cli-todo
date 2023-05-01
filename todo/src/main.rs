// TODO:
// Allow user to add, complete and delete Tasks, as well as displaying their status and number of completed vs incomplete tasks
// Add todo --help configuration for help/usage, todo --display for displaying List

use todo::{take_command, List, Status, Task};

fn main() {
    // Create List
    let mut list = List::new();
    println!("\nTodo: 0 completed tasks. 0 incomplete tasks.\n-----\n\nEnter command:");

    loop {
        // Obtain the command
        let (command, name) = take_command();

        // Exit program
        if command == "exit" {
            println!("Exiting program...");
            break;
        }

        // Add new task if it does not already exist
        if command == "add"
            && !list.items.iter().any(|task| {
                task.name == name
                    && (task.status == Status::Complete || task.status == Status::Incomplete)
            })
        {
            let task = Task::new(name.clone());
            list.add_task(task);
        }

        // If the command is "complete" and name exists for a Task within the List
        if command == "complete"
            && list
                .items
                .iter()
                .any(|task| task.name == name && task.status == Status::Incomplete)
        {
            // Update the list by mapping through it's Tasks
            list.items = list
                .items
                .iter()
                .map(|task| {
                    // Set the status to Complete if the name matches
                    if task.name == name {
                        Task {
                            name: name.clone(),
                            status: Status::Complete,
                        }
                    // Return original ask if the name does not match
                    } else {
                        Task {
                            name: task.name.clone(),
                            status: task.status.clone(),
                        }
                    }
                })
                .collect();
                list.calculate_items();
        }

        println!("{}", list);
    }

    // TODO:
    // Only add item if it does not already exist in List
    // Only complete item if it has been added
    // Can delete item if it has been completed or deleted

    ();
}
