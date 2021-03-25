use std::io;

fn main() {
let mut choice = String::new();

loop {
    calculator();
    println!("Do you want to continue? Enter 'Y' to continue and 'N' to exit");
    io::stdin()
    .read_line(&mut choice)
    .expect("Y to continue and N to exit");

    if choice.trim() == "N" {
        break;
    }
}
   
}

fn calculator() {
    let mut first_number_string = String::new();
    let mut second_number_string = String::new();
    let mut operation = String::new();

    println!("Enter first number");
    io::stdin()
        .read_line(&mut first_number_string)
        .expect("Error in first number");
    println!("Enter second number");
    io::stdin()
        .read_line(&mut second_number_string)
        .expect("Error in second number");

    let first_number: f64 = first_number_string
        .trim()
        .parse()
        .expect("Error in parsing first number");
    let second_number: f64 = second_number_string
        .trim()
        .parse()
        .expect("Error in parsing second number");

    println!("Which operation you want to do? Please choose from '+', '-', '*', '/'");
    io::stdin()
        .read_line(&mut operation)
        .expect("Invalid Operator. Please choose from '+', '-', '*', '/'");

    if operation.trim() == "+" {
        println!("The result is: {}", first_number + second_number)
    } else if operation.trim() == "-" {
        println!("The result is: {}", first_number - second_number)
    } else if operation.trim() == "/" {
        println!("The result is: {}", first_number / second_number)
    } else if operation.trim() == "*" {
        println!("The result is: {}", first_number * second_number)
    } else {
        println!("Invalid Operator. Please choose from '+', '-', '*', '/'");
    }
}
