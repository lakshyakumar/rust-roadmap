// 3. How do you implement the From and Into traits for a new type UserId(u64)? Demonstrate converting between u64 and UserId using .into(). Why are these traits important for ergonomic type conversions in Rust?
struct UserId(u64);

impl From<u64> for UserId {
    fn from(a: u64) -> Self {
        UserId(a)
    }
}

fn main() {
    let mut input = String::new();
    println!("what's the number you want to add to the type UserId?");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the number");

    let a: u64 = input.trim().parse().unwrap();
    let ui = UserId::from(a);
    println!("User Id object is: UserId({})", ui.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let a = UserId::from(32u64);
        assert_eq!(a.0, 32);
    }

    #[test]
    fn test_into() {
        let a: UserId = 32.into();
        assert_eq!(a.0, 32);
    }
}
