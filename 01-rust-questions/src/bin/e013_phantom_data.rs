// 13. What is PhantomData and how is it used?
// Demonstrate with a TypedId<T>(u64, PhantomData<T>) to create distinct types like UserId and ProductId that cannot be intermixed.
// Why is this pattern useful for type safety?
use std::marker::PhantomData;

struct TypedId<T> {
    id: u64,
    _marker: PhantomData<T>,
}

// Example types
struct User;
struct Product;

// Type aliases for clarity
type UserId = TypedId<User>;
type ProductId = TypedId<Product>;

impl<T> TypedId<T> {
    fn new(id: u64) -> Self {
        TypedId {
            id,
            _marker: PhantomData,
        }
    }
}

fn main() {
    use std::io::{self, Write};

    println!("Enter user id (u64):");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let user_id_val: u64 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };
    let user_id = UserId::new(user_id_val);

    println!("Enter product id (u64):");
    input.clear();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let product_id_val: u64 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };
    let product_id = ProductId::new(product_id_val);

    println!("UserId: {}", user_id.id);
    println!("ProductId: {}", product_id.id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_userid_creation() {
        let uid = UserId::new(123);
        assert_eq!(uid.id, 123);
    }

    #[test]
    fn test_productid_creation() {
        let pid = ProductId::new(456);
        assert_eq!(pid.id, 456);
    }

    #[test]
    fn test_type_safety() {
        let uid = UserId::new(1);
        let pid = ProductId::new(2);
        // Compile-time type safety: Uncommenting below should fail to compile
        // let x: UserId = pid;
        assert_ne!(uid.id, pid.id);
    }
}
