// 95. How do you ensure security in Rust by using constant-time equality for secrets,
// forbidding unsafe except in specific modules, and auditing dependencies?
// What are the best practices for secure Rust code?
use subtle::ConstantTimeEq;

fn check_secret(secret: &[u8], input: &[u8]) -> bool {
    secret.ct_eq(input).into() // returns true only if equal, constant-time
}

fn main() {
    let secret = b"supersecret";
    let user_input = b"guess";

    if check_secret(secret, user_input) {
        println!("Access granted!");
    } else {
        println!("Access denied!");
    }
}
