use std::io;

fn main() {
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
        1 => println!("Add Task selected"),
        2 => println!("List Task selected"),
        3 => println!("Complete Task selected"),
        4 => println!("Deleted Task selected"),
        _ => println!("Invalid option"),
    }
}
