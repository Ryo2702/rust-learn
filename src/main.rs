use std::{io, task};

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    loop {
        println!("=====================");
        println!("   Task Manager");
        println!("=====================");

        println!();

        println!("1. Add Task");
        println!("2. List Task");
        println!("3. Complete Task");
        println!("4. Delete Task");
        println!("5. Exit");
        println!("\n\n");

        println!("Choose an option:");

        let mut input = String::new(); //become mutable
        io::stdin().read_line(&mut input).unwrap();
        // let input = input.trim(); //this is called shadowing
        let choice: u32 = input.trim().parse().unwrap();

        println!("You selected: {}", choice);

        match choice {
            1 => {
                println!();
                println!("Enter Task: ");

                let mut task = String::new();

                io::stdin()
                    .read_line(&mut task)
                    .unwrap();

                let task = task.trim().to_string();

                tasks.push(task);

                println!("task added");
            }
            2 => {
                println!();
                println!("List Task selected");
            }
            3 => {
                println!();
                println!("Complete Task selected");
            }
            4 => {
                println!();
                println!("Deleted Task selected");
            }
            5 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}
