// 10. How do you implement a function fn middle<T>(slice: &[T]) -> Option<&T> that returns the middle element of a slice?
// Test your function with both strings and integers. Discuss how Rust's slice API helps with safe indexing.

fn middle<T>(slice: &[T]) -> Option<&T> {
    let length = slice.len();
    if length > 0 {
        return Some(&slice[(length) / 2]);
    }
    None
}

fn main() {
    println!("Enter the comma separated array of values");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error while reading array");
    let arr: Vec<&str> = input.split(",").map(|x| x.trim()).collect();
    println!("mid element of {:?} is {}", &arr, middle(&arr).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_mid() {
        let s = "abcd";
        let chars: Vec<char> = s.chars().collect();
        assert_eq!(*middle(&chars).unwrap(), 'c');
    }

    #[test]
    fn test_arr_mid() {
        let s = [1, 2, 3, 4, 5];
        assert_eq!(middle(&s), Some(&3));
    }

    #[test]
    fn test_edge_case_mid() {
        let s: [u32; 0] = [];
        assert_eq!(middle(&s), None);
    }
}
