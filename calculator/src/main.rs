use regex::Regex;
use std::io;

fn main() {
    println!("Welcome to the calculator!");

    loop {
        println!("Enter an expression to calculate:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        if input == "exit" {
            println!("Goodbye!");
            break;
        }

        match calculate(input) {
            Some(result) => println!("Result: {}", result),
            None => println!("Invalid expression. Please try again."),
        }
    }
}

fn calculate(input: &str) -> Option<f64> {
    let re = Regex::new(r"([\d.]+)\s*([+-/*])\s*([\d.]+)").unwrap();
    if let Some(captures) = re.captures(input) {
        let num1: f64 = captures[1].parse().ok()?;
        let num2: f64 = captures[3].parse().ok()?;
        let operation = &captures[2];

        let result = match operation {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero");
                    return None;
                }
                num1 / num2
            }
            _ => return None,
        };
        Some(result)
    }else {
        return None
    }
}
