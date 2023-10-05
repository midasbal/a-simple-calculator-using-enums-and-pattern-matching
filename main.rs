use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => {
            if y != 0.0 {
                x / y
            } else {
                println!("Error: Division by zero!");
                std::f64::NAN
            }
        }
    }
}

fn main() {
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let first_number: f64 = input.trim().parse().expect("Invalid input");

    println!("Enter the operation (+, -, *, /):");
    let mut operation_input = String::new();
    io::stdin()
        .read_line(&mut operation_input)
        .expect("Failed to read line");
    input.clear();
    let operation = operation_input.trim();

    println!("Enter the second number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let second_number: f64 = input.trim().parse().expect("Invalid input");

    let operation_enum = match operation {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    let result = calculate(operation_enum);
    println!("Result: {}", result);
}
