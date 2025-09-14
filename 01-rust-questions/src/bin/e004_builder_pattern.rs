// 4. Describe how to design a configuration struct with optional fields using the builder pattern in Rust.
// How would you implement Default and chain setters that return Self? What are the benefits of using the builder pattern for complex configuration objects?

use core::fmt;
use std::io::stdin;
struct User {
    name: String,
    age: u16,
    balance: u64,
}

impl User {
    fn new(name: String) -> Self {
        Self {
            name,
            age: 18,
            balance: 0,
        }
    }

    fn set_age(mut self, age: u16) -> Self {
        self.age = age;
        self
    }

    fn set_balance(mut self, balance: u64) -> Self {
        self.balance = balance;
        self
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "user details:\nname: {},\nage: {},\nbalance :{}",
            self.name, self.age, self.balance
        )
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter the name of the user: ");
    stdin()
        .read_line(&mut input)
        .expect("not able to read name");
    let mut user = User::new(input.trim().to_string());
    let mut input = String::new();
    println!("Enter the age of user");
    stdin().read_line(&mut input).expect("not able to read age");
    user = user.set_age(input.trim().parse::<u16>().expect("Invalid age input"));

    let mut input = String::new();
    println!("Enter the balance of user");
    stdin()
        .read_line(&mut input)
        .expect("not able to read balance");
    user = user.set_balance(input.trim().parse::<u64>().expect("Invalid balance"));

    println!("The user we build is {}", user);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_name() {
        let user = User::new("bob".to_string());
        assert!(user.name == "bob");
    }

    #[test]
    fn build_age() {
        let user = User::new("bob".to_string()).set_age(32);
        assert!(user.age == 32);
    }

    #[test]
    fn build_balance() {
        let user = User::new("bob".to_string()).set_age(32).set_balance(1000);
        assert!(user.balance == 1000);
    }
}
