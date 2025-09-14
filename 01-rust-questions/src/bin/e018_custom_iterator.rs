// 18. How do you create a custom iterator in Rust? Implement a Counter with new(start, step) that yields successive values and implements Iterator for u64.
// Why is implementing custom iterators useful?

use std::io;

struct Counter {
    current: u64,
    step: u64,
}

impl Counter {
    fn new(start: u64, step: u64) -> Self {
        Counter {
            current: start,
            step,
        }
    }
}

impl Iterator for Counter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current;
        self.current = self.current.saturating_add(self.step); // safe add
        Some(value)
    }
}

fn main() {
    println!("Enter start, step, and count (comma-separated):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Split by comma, trim, and parse
    let parts: Vec<u64> = input
        .split(',')
        .map(|s| s.trim().parse::<u64>().expect("Invalid number"))
        .collect();

    if parts.len() != 3 {
        eprintln!("Please enter exactly 3 values: start, step, count");
        return;
    }

    let (start, step, count) = (parts[0], parts[1], parts[2]);

    println!(
        "Generating {} numbers starting at {} with step {}:",
        count, start, step
    );

    let counter = Counter::new(start, step);
    for n in counter.take(count as usize) {
        println!("{}", n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_counter() {
        let mut counter = Counter::new(0, 1);
        assert_eq!(counter.next(), Some(0));
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
    }

    #[test]
    fn test_custom_step() {
        let mut counter = Counter::new(5, 3);
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), Some(8));
        assert_eq!(counter.next(), Some(11));
    }

    #[test]
    fn test_take_adapter() {
        let counter = Counter::new(10, 2);
        let collected: Vec<u64> = counter.take(5).collect();
        assert_eq!(collected, vec![10, 12, 14, 16, 18]);
    }

    #[test]
    fn test_large_values_saturating() {
        let mut counter = Counter::new(u64::MAX - 1, 5);
        assert_eq!(counter.next(), Some(u64::MAX - 1));
        // next value would overflow, but saturating_add prevents panic
        assert_eq!(counter.next(), Some(u64::MAX));
        assert_eq!(counter.next(), Some(u64::MAX)); // stays at max forever
    }
}
