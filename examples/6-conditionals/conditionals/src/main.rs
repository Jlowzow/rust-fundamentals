// There are other conditionals that we can explore in Rust. Like using `if let`

fn main() {
    let maybe_number:Option<Option<Option<()>>> = Some(Some(None));
    //let maybe_number: Option<Option<()>> = Some(None);
    //let maybe_number = Some(42);
    if let Some(number) = maybe_number {
        println!("The number is {:?}", number);
        if let Some(number) = number {
            println!("The value is {:?}", number);
        } else {
            println!("There is no value");
        }
    } else {
        println!("There is no number");
    }
}
