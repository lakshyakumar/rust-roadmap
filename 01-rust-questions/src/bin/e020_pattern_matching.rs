// 20. What are pattern matching ergonomics in Rust? Show how to match on &Option<String> to avoid cloning, using ref and ref mut bindings.
// Why is ergonomic pattern matching important for performance?
fn main() {
    let mut name: Option<String> = Some("Alice".to_string());

    // --- match ---
    match &name {
        Some(ref s) => println!("match: found name = {}", s),
        None => println!("match: no name"),
    }

    // --- if let ---
    if let Some(ref s) = &name {
        println!("if let: found name = {}", s);
    }

    // --- matches! ---
    if matches!(&name, Some(_)) {
        println!("matches!: there is a name");
    }

    // --- mutable match ---
    match &mut name {
        Some(ref mut s) => {
            s.push_str(" Smith");
            println!("match (mutable): updated name = {}", s);
        }
        None => println!("match (mutable): no name"),
    }

    println!("Final value: {:?}", name);
}
