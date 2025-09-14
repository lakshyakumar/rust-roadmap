struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter::new();

    for num in counter {
        println!("{}", num);
    }

    let sum_of_even_squares: u32 = Counter::new().map(|x| x * x).filter(|x| x % 2 == 0).sum();
    println!("Sum of even squares: {}", sum_of_even_squares);
}
