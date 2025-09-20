// 94. How do you use cargo fuzz to fuzz your parser, fix discovered panics or UB, and add regression tests?
// Why is fuzzing important for security?
// single_file_fuzz.rs
// -------------------
// To run fuzzing: `cargo fuzz run parser` after creating a fuzz project
// To run unit tests: `cargo test`

use std::str;

/// ---------------------------
/// Example parser
/// ---------------------------
pub fn parse_custom(input: &str) -> Result<(), String> {
    if input.is_empty() {
        return Err("empty input".into());
    }
    if input.contains('!') {
        return Err("invalid character '!'".into()); // avoid panic
    }
    if input.len() > 1_000_000 {
        return Err("input too long".into()); // avoid out-of-bounds
    }
    Ok(())
}

/// ---------------------------
/// Fuzz target (for cargo-fuzz)
/// ---------------------------
#[cfg(fuzzing)]
mod fuzz_target {
    use super::*;
    use libfuzzer_sys::fuzz_target;

    fuzz_target!(|data: &[u8]| {
        if let Ok(s) = str::from_utf8(data) {
            let _ = parse_custom(s);
        }
    });
}

/// ---------------------------
/// Unit regression tests
/// ---------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regression_empty() {
        let input = "";
        assert!(parse_custom(input).is_err());
    }

    #[test]
    fn regression_exclamation() {
        let input = "hello!";
        assert!(parse_custom(input).is_err());
    }

    #[test]
    fn regression_long_input() {
        let input = "a".repeat(2_000_000);
        assert!(parse_custom(&input).is_err());
    }

    #[test]
    fn regression_valid_input() {
        let input = "hello world";
        assert!(parse_custom(input).is_ok());
    }
}

/// ---------------------------
/// Optional main for manual testing
/// ---------------------------
fn main() {
    let test_inputs = vec![
        "".to_string(),
        "hello!".to_string(),
        "a".repeat(2_000_000),
        "valid".to_string(),
    ];
    for input in &test_inputs {
        match parse_custom(input) {
            Ok(_) => println!("OK: '{}'", &input.chars().take(20).collect::<String>()),
            Err(e) => println!(
                "Err: '{}' -> {}",
                &input.chars().take(20).collect::<String>(),
                e
            ),
        }
    }
}
