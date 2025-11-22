
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn avg(numbers: &[i32]) -> f64 {
    let total = sum(numbers) as f64;
    let count = numbers.len() as f64;
    if count == 0.0 {
        0.0
    } else {
        total / count
    }
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
    println!("The average is {}", avg(&input_numbers));
}
