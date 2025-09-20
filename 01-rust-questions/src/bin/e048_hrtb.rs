// 48. What are higher-ranked trait bounds (HRTB)?
// Write a function fn apply<'a, F>(f: F) where F: for<'b> Fn(&'b str) -> &'b str and demonstrate its use.
// Why are HRTBs needed?

fn apply<F>(f: F)
where
    F: for<'b> Fn(&'b str) -> &'b str,
{
    let s1 = String::from("hello");
    let s2 = String::from("world");

    println!("{}", f(&s1));
    println!("{}", f(&s2));
}

// Identity function works for all lifetimes
fn identity<'a>(x: &'a str) -> &'a str {
    x
}

fn main() {
    apply(identity);
}
