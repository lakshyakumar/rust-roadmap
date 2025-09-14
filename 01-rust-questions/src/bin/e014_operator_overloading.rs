// 14. How do you overload operators in Rust? Implement Add for a Point struct so that p1 + p2 works,
// and explain the required trait and associated type. What are the implications for custom types?

use std::{
    fmt::{self, Display},
    ops::Add,
};

struct Coordinate<T> {
    x: T,
    y: T,
}

impl<T> Coordinate<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Add for Coordinate<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Coordinate<T>;

    fn add(self, coordinate: Coordinate<T>) -> Self::Output {
        Coordinate {
            x: self.x + coordinate.x,
            y: self.y + coordinate.y,
        }
    }
}

impl<T> Display for Coordinate<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", &self.x, &self.y)
    }
}

fn main() {
    use std::io::{self, Write};
    println!("Enter first coordinate as x,y:");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let nums: Vec<&str> = input.trim().split(',').collect();
    if nums.len() != 2 {
        println!("Invalid input. Please enter as x,y");
        return;
    }
    let x1: i32 = match nums[0].trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid x");
            return;
        }
    };
    let y1: i32 = match nums[1].trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid y");
            return;
        }
    };

    println!("Enter second coordinate as x,y:");
    input.clear();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let nums: Vec<&str> = input.trim().split(',').collect();
    if nums.len() != 2 {
        println!("Invalid input. Please enter as x,y");
        return;
    }
    let x2: i32 = match nums[0].trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid x");
            return;
        }
    };
    let y2: i32 = match nums[1].trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid y");
            return;
        }
    };

    let a = Coordinate::new(x1, y1);
    let b = Coordinate::new(x2, y2);
    let c = a + b;
    println!("c is {}", c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_coordinates() {
        let a = Coordinate::new(1, 2);
        let b = Coordinate::new(3, 4);
        let c = a + b;
        assert_eq!(c.x, 4);
        assert_eq!(c.y, 6);
    }

    #[test]
    fn test_display() {
        let a = Coordinate::new(5, 7);
        assert_eq!(format!("{}", a), "{ x: 5, y: 7 }");
    }
}
