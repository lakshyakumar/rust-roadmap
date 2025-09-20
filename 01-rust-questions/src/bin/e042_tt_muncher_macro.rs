// 42. How do you write a declarative macro (TT-muncher) like hashmap! { k => v, ... } that builds a HashMap and supports trailing commas?
// Why is macro flexibility important?

// A TT-muncher (Token Tree muncher) is a style of macro_rules! macro that recursively consumes tokens until none are left.
// It’s useful for handling lists (like key => value, key => value, ...) and also for supporting things like trailing commas.

#[macro_export]
macro_rules! hashmap {
    // Base case: no entries left
    () => {{
        ::std::collections::HashMap::new()
    }};

    // Recursive case: handle one key-value pair, then recurse
    ($($k:expr => $v:expr),+ $(,)?) => {{
        let mut _map = ::std::collections::HashMap::new();
        $(
            _map.insert($k, $v);
        )+
        _map
    }};
}

fn main() {
    let m = hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };

    println!("{:?}", m); // {"one": 1, "two": 2, "three": 3}
}
