// 6. How can you compute a moving average over a Vec<f64> using iterator adapters like windows and map?
// Return the result as a Vec<f64>. Discuss the efficiency of this approach compared to manual iteration.

fn moving_averages(items: &Vec<f64>, window_size: usize) -> Vec<f64> {
    println!("{:?},{}", items, window_size);
    items
        .windows(window_size)
        .map(|w| w.iter().sum::<f64>() / w.len() as f64)
        .collect()
}

fn main() {
    let mut input = String::new();
    println!("Enter the numbers separated by ',':");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the numbers");
    let a: Vec<f64> = input
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    println!("Enter the window size: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading window size");
    let win_size = input.trim().parse().expect("Error parsing the Window Size");

    println!(
        "Moving average for arr: {:?}, with window size as {}, is {:?}",
        a.clone(),
        win_size,
        moving_averages(&a, win_size)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_average() {
        assert_eq!(
            moving_averages(&vec![2.0, 3.0, 4.0, 5.0], 2),
            vec![2.5, 3.5, 4.5]
        );
    }
}
