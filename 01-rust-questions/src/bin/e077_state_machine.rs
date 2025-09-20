// 77. How do you implement a non-blocking parser as a state machine enum in Rust?
// Advance the state with advance(&mut self, byte) -> Option<Token>. What are the advantages of state machines?

// A state machine parser processes input byte by byte, maintaining state across calls.
// This makes it:
// Non-blocking → You can feed data incrementally (great for streams, sockets, async I/O).
// Memory efficient → No need to buffer the whole input.
// Deterministic & explicit → Each state encodes exactly what’s expected next.
// Composable → You can layer small parsers (numbers, strings, keywords).
// This is how HTTP parsers, JSON streaming parsers, protocol decoders are often built.

fn main() {}
