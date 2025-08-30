use std::io;

fn main() {
    welcome_message();

    let mut input = -1; //dummy to enter the loop

    while input != 0 {
        output_actions();
        input = get_user_input();

        match input {
            1 => create_file(),
            2 => edit_file(),
            3 => delete_file(),
            0 => exit_message(),
            _ => println!("\nPlease enter an option 1, 2, 3, or 0"),
        }
    }
}

fn get_user_input() -> i8 {
    // declare a string to read from the command line
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    // convert the string to a int
    let user_input = user_input
        .trim()
        .parse::<i8>()
        .expect("Please Input a Number");

    return user_input;
}

fn create_file() {
    println!("\nCreating a new file...");
}

fn edit_file() {
    println!("\nEdit file option selected!");
}

fn delete_file() {
    println!("\ndelete file option selected!");
}

fn welcome_message() {
    // welcome the user and open the loop for user input
    println!("===========================");
    println!("Welcome to Rust Note Taker!");
    println!("===========================");
}

fn output_actions() {
    println!("\nPlease Select an Option:");
    println!("1: Create a new file");
    println!("2: Edit a file");
    println!("3: Delete a file");
    println!("0: Exit");
}

fn exit_message() {
    println!("\nGoodbye!");
}
