// 1. How can you take a Vec<i32> and return a Vec<i32> containing only the squares of the even numbers, using iterator methods (iter, filter, map, collect) without using explicit loops? Explain the advantages of using iterator combinators over traditional loops in terms of performance and readability.

use std::io;

fn squared_filter(a: Vec<i32>)->Vec<i32>{
    a.iter().filter(|x| *x%2==0).map(|x| x*x).collect()
}

fn main() {
    let mut input = String::new();
    println!("Enter the comma separated numbers:");
    io::stdin().read_line(&mut input).expect("Failed to read lines");
    let a : Vec<i32> = input.trim().split(",").map(|x| {
        let i = x.trim();
        i.parse::<i32>().expect("NAN")
    }).collect();

    println!("Input we got from user {:?} and squared array is:: {:?}", a.clone(), squared_filter(a));
    
}

#[cfg(test)]
mod tests{
    use super::*;

        #[test]
        fn test_squares_of_even_numbers() {
            let input = vec![1, 2, 3, 4, 5];
            let expected = vec![4, 16];
            assert_eq!(squared_filter(input), expected);
        }

        #[test]
        fn test_empty_vec() {
            let input: Vec<i32> = vec![];
            let expected: Vec<i32> = vec![];
            assert_eq!(squared_filter(input), expected);
        }

        #[test]
        fn test_no_even_numbers() {
            let input = vec![1, 3, 5, 7];
            let expected: Vec<i32> = vec![];
            assert_eq!(squared_filter(input), expected);
        }

        #[test]
        fn test_all_even_numbers() {
            let input = vec![2, 4, 6];
            let expected = vec![4, 16, 36];
            assert_eq!(squared_filter(input), expected);
        }
}
