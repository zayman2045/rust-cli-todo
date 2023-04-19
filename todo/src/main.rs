// TODO:
// Allow user to add, complete and delete Tasks, as well as displaying their status and number of completed vs incomplete tasks
// Add todo --help configuration for help/usage, todo --display for displaying List

use todo::{take_command, List, Task, Status};



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




