// 78. How do you implement a Base64 encoder/decoder using iterator adapters in Rust?
// Test your implementation against the base64 crate for correctness. Why is iterator-based encoding efficient?

// Zero-copy style: Operates directly on streams of bytes instead of allocating intermediate buffers.
// Lazy evaluation: You can encode/decode data as you produce/consume it (important for large streams, sockets, files).
// Pipeline composition: Works seamlessly with other iterators (map, filter, flat_map).
// Cache-friendly: Processes data chunk by chunk with minimal branching.

// main.rs
const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Encode bytes into Base64 using iterator adapters
fn encode_base64(input: &[u8]) -> String {
    let mut out = String::new();

    for chunk in input.chunks(3) {
        let b0 = chunk.get(0).copied().unwrap_or(0);
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);

        let n = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);

        let indices = [
            ((n >> 18) & 0x3F) as u8,
            ((n >> 12) & 0x3F) as u8,
            ((n >> 6) & 0x3F) as u8,
            (n & 0x3F) as u8,
        ];

        for (i, &idx) in indices.iter().enumerate() {
            if i < chunk.len() + 1 {
                out.push(BASE64_TABLE[idx as usize] as char);
            } else {
                out.push('=');
            }
        }
    }

    out
}

/// Decode Base64 using iterator adapters
fn decode_base64(input: &str) -> Result<Vec<u8>, &'static str> {
    let mut out = Vec::new();
    let mut buf = [0u32; 4];

    let mut iter = input.chars().filter(|&c| c != '=').map(|c| {
        BASE64_TABLE
            .iter()
            .position(|&b| b as char == c)
            .ok_or("Invalid character")
    });

    loop {
        for i in 0..4 {
            match iter.next() {
                Some(Ok(v)) => buf[i] = v as u32,
                Some(Err(e)) => return Err(e),
                None => {
                    if i == 0 {
                        return Ok(out); // done
                    } else {
                        return Err("Invalid padding length");
                    }
                }
            }
        }

        let n = (buf[0] << 18) | (buf[1] << 12) | (buf[2] << 6) | buf[3];
        out.push(((n >> 16) & 0xFF) as u8);
        out.push(((n >> 8) & 0xFF) as u8);
        out.push((n & 0xFF) as u8);
    }
}

fn main() {
    let input = b"foobar";
    let encoded = encode_base64(input);
    let decoded = decode_base64(&encoded).unwrap();

    println!("Input:   {:?}", String::from_utf8_lossy(input));
    println!("Encoded: {}", encoded);
    println!("Decoded: {:?}", String::from_utf8_lossy(&decoded));
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64;

    #[test]
    fn test_encode_decode() {
        let inputs = vec![
            b"".as_ref(),
            b"f",
            b"fo",
            b"foo",
            b"foob",
            b"fooba",
            b"foobar",
        ];

        for &input in &inputs {
            let ours = encode_base64(input);
            let theirs = base64::encode(input);
            assert_eq!(ours, theirs);

            let ours_dec = decode_base64(&ours).unwrap();
            let theirs_dec = base64::decode(&theirs).unwrap();
            assert_eq!(ours_dec, theirs_dec);
        }
    }
}
