
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    // There are no variadic arguments in Rust

    // take keyboard input to fill the array of arbitrary integers  
    use std::io;
    println!("Enter any number of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input_numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let result = sum(&input_numbers);
    println!("The sum is {}", result);
}
