// 5. How do you handle errors in Rust using Result, the ? operator, and the thiserror crate?
// Write a function read_config(path) -> Result<Config, ConfigError> and define a custom error type.
// Explain how error propagation works and why custom error types are useful.

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
enum CalcError {
    #[error("Overflow: by {a} {operator} {b}, exceeding the range of (0-255)")]
    Overflow { a: u8, b: u8, operator: char },

    #[error("Trying to divide by 0")]
    DivisionByZero,
}

#[derive(Debug, PartialEq)]
enum CalcResult {
    Int(u8),
    Float(f64),
}

fn add(a: u8, b: u8) -> Result<CalcResult, CalcError> {
    a.checked_add(b)
        .map(CalcResult::Int)
        .ok_or(CalcError::Overflow {
            a,
            b,
            operator: '+',
        })
}

fn divide(a: u8, b: u8) -> Result<CalcResult, CalcError> {
    if b == 0 {
        return Err(CalcError::DivisionByZero);
    }

    if a % b == 0 {
        Ok(CalcResult::Int(a / b))
    } else {
        Ok(CalcResult::Float(a as f64 / b as f64))
    }
}

// ...existing code...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_success() {
        assert_eq!(add(10, 20), Ok(CalcResult::Int(30)));
    }

    #[test]
    fn test_add_overflow() {
        let result = add(200, 100);
        match result {
            Err(CalcError::Overflow { a, b, operator }) => {
                assert_eq!(a, 200);
                assert_eq!(b, 100);
                assert_eq!(operator, '+');
            }
            _ => panic!("Expected overflow error"),
        }
    }

    #[test]
    fn test_divide_success_int() {
        assert_eq!(divide(20, 5), Ok(CalcResult::Int(4)));
    }

    #[test]
    fn test_divide_success_float() {
        assert_eq!(divide(7, 2), Ok(CalcResult::Float(3.5)));
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10, 0);
        match result {
            Err(CalcError::DivisionByZero) => {}
            _ => panic!("Expected division by zero error"),
        }
    }
}

fn main() {
    use std::io::{self, Write};
    println!("Enter two numbers separated by ',' (e.g. 10,20):");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let nums: Vec<&str> = input.trim().split(',').collect();
    if nums.len() != 2 {
        println!("Invalid input. Please enter two numbers separated by a comma.");
        return;
    }
    let a = match nums[0].trim().parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid first number.");
            return;
        }
    };
    let b = match nums[1].trim().parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid second number.");
            return;
        }
    };

    println!("Enter operator (+ or %):");
    let mut op = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut op)
        .expect("Failed to read operator");
    let op = op.trim();

    let result = match op {
        "+" => add(a, b),
        "%" => divide(a, b),
        _ => {
            println!("Unsupported operator. Use + or %.");
            return;
        }
    };

    match result {
        Ok(CalcResult::Int(n)) => println!("Result: {}", n),
        Ok(CalcResult::Float(f)) => println!("Result: {:.4}", f),
        Err(e) => println!("Error: {}", e),
    }
}
