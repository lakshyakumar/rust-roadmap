// 63. How do you use proptest for property-based testing in Rust?
// Test a reversible encode/decode pair and shrink failing cases. Why is property-based testing valuable?

// If the property fails, proptest will try to shrink the failing input to the smallest counterexample.
// Example: If vec![13, 255, 42] breaks the property, proptest will test smaller inputs ([13, 255], [13], []) to find the minimal failing case.
// This makes debugging much easier.

use base64::{engine::general_purpose, Engine as _};
use proptest::prelude::*;

/// Our encode function
fn encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

/// Our decode function
fn decode(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(s)
}

proptest! {
    // Property: decode(encode(x)) == x
    #[test]
    fn encode_decode_roundtrip(input in proptest::collection::vec(any::<u8>(), 0..100)) {
        let encoded = encode(&input);
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, input);
    }


}

fn main() {}
