// 43. What is macro hygiene in Rust? Write a log_expr! macro that evaluates an expression once and logs its value,
// using let bindings for hygiene. How does hygiene prevent bugs?

// Macro hygiene is the compiler feature that prevents identifiers (variables, functions, types) introduced by a macro from
//  accidentally colliding with identifiers in the code that uses the macro — and vice versa.
// In short: names created inside a macro are separate (hygienic) by default, so the macro can't unexpectedly capture or be captured by names from the caller.
//  That makes macros safer and reduces a large class of bugs.

#[macro_export]
macro_rules! log_expr {
    ($e:expr) => {{
        // temporary binding created inside the macro
        let __log_expr_val = $e;
        // log file, line, the expression text, and debug-format the value
        println!(
            "[{}:{}] {} = {:?}",
            file!(),
            line!(),
            stringify!($e),
            &__log_expr_val
        );
        // return the value
        __log_expr_val
    }};
}

fn incr(x: &mut i32) -> i32 {
    *x += 1;
    *x
}

fn main() {
    let mut a = 0;

    // expression evaluated once; logged; value returned
    let v = log_expr!(incr(&mut a) + 5);
    // prints something like: "[src/main.rs:12] incr(&mut a) + 5 = 6"
    println!("v = {}, a = {}", v, a); // v = 6, a = 1
}
