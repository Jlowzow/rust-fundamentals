use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");


    // use of match expression to pattern match against variable "name"
    // change to case insensitive matching
    match name.trim().to_ascii_lowercase().as_str() {
        "good bye" => println!("Sorry to see you go."),
        "hello" => println!("Hi, nice to meet you!"),
        "how are you?" => println!("I'm doing well, thank you!"),
        "where are you going?" => println!("I'm heading to the park."),
        _ => println!("I can't find a greeting, good bye."),
    }
}