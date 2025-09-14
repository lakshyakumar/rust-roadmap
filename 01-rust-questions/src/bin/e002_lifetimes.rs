// 2. What are lifetimes in Rust? Write a function fn longer<'a>(a: &'a str, b: &'a str) -> &'a str with explicit lifetime annotations and explain why lifetime elision does not apply here. Discuss how lifetimes help prevent dangling references and memory safety issues.

use std::io::stdin;

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    }
    b
}

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter first string: ");
    stdin()
        .read_line(&mut input1)
        .expect("Failed to read string 1");
    println!("Enter second string:");
    stdin()
        .read_line(&mut input2)
        .expect("Failed to read string 2");

    println!(
        "the longer string from {} and {} is {}",
        &input1.trim(),
        &input2.trim(),
        longer(&(input1.trim()), &(input2.trim()))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_longer_string() {
        let a = String::from("abcd");
        let b = String::from("abc");
        assert_eq!(longer(&a, &b), &a);
    }

    #[test]
    fn test_second_longer_string() {
        let a = String::from("abcd");
        let b = String::from("abcde");
        assert_eq!(longer(&a, &b), &b);
    }

    #[test]
    fn test_both_longer_string() {
        let a = String::from("abcde");
        let b = String::from("abcde");
        assert_eq!(longer(&a, &b), &b);
    }
}
