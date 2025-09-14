// 11. What are generic constraints in Rust?
// Write a function fn print_both<T: std::fmt::Display + std::fmt::Debug>(t: &T) that prints formatted output.
// Why are trait bounds important for generic programming?

#[derive(Debug)]
struct Score(u128);

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "score is {}", self.0)
    }
}

fn print_both<T: std::fmt::Display + std::fmt::Debug>(t: &T) {
    println!("{}", t);
    println!("{:?}", t);
}

fn main() {
    let score = Score(32);
    print_both(&score);
}
