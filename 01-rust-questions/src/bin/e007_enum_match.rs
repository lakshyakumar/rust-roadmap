// 7. Define an enum Shape and implement an area() method. How would you use the matches! macro to count only Circle variants in a slice?
// Explain the use of pattern matching and macros for concise code.

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match &self {
            Shape::Circle(r) => 3.14 * r * r,
            Shape::Rectangle(a, b) => a * b,
        }
    }
}

fn main() {
    let mut input = String::new();
    println!("select a shape from one of the below:\n1. Circle\n2. Rectangle");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Cannot read the option");
    let shape = input
        .trim()
        .parse::<u8>()
        .expect("Not able to parse response");

    input.clear();

    if shape == 1 {
        println!("You chose a circle");
        println!("Enter the radius of the circle:");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Cannot read the option");
        println!(
            "Area of a circle with radius {}",
            Shape::Circle(input.trim().parse::<f64>().expect("Error reading radius")).area()
        );
    } else if shape == 2 {
        println!("you chose a rectangle");
        println!("Enter teh length and breadth of a rectangle, separated by comma:");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Cannot read the option");
        let dim: Vec<f64> = input
            .split(",")
            .map(|a| a.trim().parse::<f64>().unwrap())
            .collect();
        println!(
            "Area of rectangle is: {}",
            Shape::Rectangle(dim[0], dim[1]).area()
        );
    } else {
        println!("invalid choice");
    }

    // match input
    //     .trim()
    //     .parse::<u8>()
    //     .expect("Not able to parse response")
    // {
    //     1 => Shape::Circle(()),
    //     2 => Shape::Rectangle((), ()),
    //     _ => println!("Incorrect response"),
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_test() {
        let c = Shape::Circle(10.0);
        let b = Shape::Rectangle(5.0, 5.0);

        assert_eq!(c.area(), 314f64);
        assert_eq!(b.area(), 25f64);
    }

    #[test]
    fn circle_counter() {
        let shapes = [
            Shape::Circle(2.0),
            Shape::Rectangle(3.0, 4.0),
            Shape::Circle(1.5),
        ];

        assert_eq!(
            shapes
                .iter()
                .filter(|shape| matches!(shape, Shape::Circle(_)))
                .count(),
            2
        )
    }
}
