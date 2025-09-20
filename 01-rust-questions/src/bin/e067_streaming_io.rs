// 67. How do you read from stdin line-by-line using BufRead::lines, transform the input, and write to stdout efficiently?
// Why is streaming I/O important?

// Why is streaming I/O important?
// Memory efficiency → You don’t load the whole input into memory (critical for large files or streams).
// Low latency → Process data as it arrives instead of waiting for the whole input.
// Scalability → Works equally well for a few KB or multi-GB logs.
// Composability → Allows chaining (e.g., piping from cat → transformer → grep).

use std::io::{self, BufRead, Write};
fn main() {
    // Lock stdin and stdout for efficiency (avoids re-locking for every line)
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle_out = stdout.lock();

    for line in stdin.lock().lines() {
        match line {
            Ok(mut text) => {
                // --- Transform the line ---
                // Example: uppercase + trim trailing whitespace
                text = text.trim_end().to_uppercase();

                // --- Write to stdout ---
                writeln!(handle_out, "{}", text).unwrap();
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }
}
