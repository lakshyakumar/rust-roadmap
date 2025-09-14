// 8. How do you build a word frequency counter for a &str using split_whitespace and HashMap's entry().or_insert(0) += 1 pattern?
// Discuss the role of entry API in efficient hashmap updates.

use std::collections::HashMap;

struct FTGenerator {
    s: String,
    map: HashMap<String, u32>,
}

impl FTGenerator {
    fn new(s: &str) -> Self {
        let mut map = HashMap::new();
        for word in s.split_whitespace() {
            *map.entry(word.to_string()).or_insert(0) += 1;
        }
        FTGenerator {
            s: s.to_string(),
            map,
        }
    }
}

fn main() {
    println!("Enter a string for the word map");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");
    let ft = FTGenerator::new(&input);

    loop {
        input.clear();
        println!("Enter the  word to be searched in map");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");
        match ft.map.get(input.trim()) {
            Some(occ) => println!("{} found {} times in the string", &input.trim(), occ),
            _ => println!("Not found in map"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let ft = FTGenerator::new("Ravi had been saying that he had been there");
        assert_eq!(*ft.map.get("had").unwrap(), 2);
        assert_eq!(*ft.map.get("been").unwrap(), 2);
        assert_eq!(*ft.map.get("saying").unwrap(), 1);
        assert_eq!(ft.map.get("not"), None);
    }
}
