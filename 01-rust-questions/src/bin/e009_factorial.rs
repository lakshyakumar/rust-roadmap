// 9. Write a function to compute the factorial of n using Iterator::fold over the range (1..=n).
// Explain why this approach is idiomatic in Rust and how it leverages functional programming concepts.

fn factorial(n: u128) -> u128 {
    (1..=n).fold(1, |acc, item| acc * item)
}

fn main() {
    let mut input = String::new();
    println!("Enter a number for factorial: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Not able to read number");
    let n: u128 = input.trim().parse().unwrap();
    println!("Factorial for {} is {}", n, factorial(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_0_factorial() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn testing_1_factorial() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn testing_5_factorial() {
        assert_eq!(factorial(5), 120);
    }
}
