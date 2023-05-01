// TODO:
// Allow user to add, complete and delete Tasks, as well as displaying their status and number of completed vs incomplete tasks
// Add todo --help configuration for help/usage, todo --display for displaying List

use todo::{take_command, List, Status, Task};

fn main() {
    // Create List
    let mut list = List::new();
    println!("\nTodo: 0 completed tasks. 0 incomplete tasks.\n-----\n\nEnter command:");

    loop {
        // Take the command for the user
        let (input_command, input_name) = take_command();

        // If the command is "exit" then exit the program
        if input_command == "exit" {
            println!("Exiting program...");
            break;
        }

        // If the command is "add" and the name does not exist within the List
        if input_command == "add"
            && !list.items.iter().any(|task| {
                task.name == input_name
                    && (task.status == Status::Complete || task.status == Status::Incomplete)
            })
        {
            let task = Task::new(input_name.clone());
            list.add_task(task);
        }

        // If the command is "complete" and name exists for an incomplete Task within the List
        if input_command == "complete"
            && list
                .items
                .iter()
                .any(|task| task.name == input_name && task.status == Status::Incomplete)
        {
            list.complete_task(&input_name);
        }

        // If the command is "delete" and the name exits for any Task within the List
        if input_command == "delete" && list.items.iter().any(|task|task.name == input_name) {
            list.delete_task(&input_name);
        }
        // filter out task to be deleted
        println!("{}", list);
    }

    ();
}
