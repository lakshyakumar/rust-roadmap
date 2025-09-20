// 64. How do you verify algebraic laws (e.g., Monoid laws) for a custom type using QuickCheck-style property tests?
// What are the benefits of law-based testing?

use proptest::prelude::*;

/// A Monoid trait: has an identity element and an associative operation
trait Monoid: Clone + PartialEq {
    fn empty() -> Self;
    fn combine(&self, other: &Self) -> Self;
}

/// Implement Monoid for Vec<T> (concatenation)
impl<T: Clone + PartialEq> Monoid for Vec<T> {
    fn empty() -> Self {
        vec![]
    }

    fn combine(&self, other: &Self) -> Self {
        let mut out = self.clone();
        out.extend_from_slice(other);
        out
    }
}

proptest! {
    #[test]
    fn monoid_left_identity(xs in proptest::collection::vec(any::<i32>(), 0..10)) {
        // ∀x: empty ∘ x == x
        let lhs = Vec::<i32>::empty().combine(&xs);
        prop_assert_eq!(lhs, xs);
    }

    #[test]
    fn monoid_right_identity(xs in proptest::collection::vec(any::<i32>(), 0..10)) {
        // ∀x: x ∘ empty == x
        let rhs = xs.combine(&Vec::<i32>::empty());
        prop_assert_eq!(rhs, xs);
    }

    #[test]
    fn monoid_associativity(xs in proptest::collection::vec(any::<i32>(), 0..5),
                            ys in proptest::collection::vec(any::<i32>(), 0..5),
                            zs in proptest::collection::vec(any::<i32>(), 0..5)) {
        // ∀x,y,z: (x ∘ y) ∘ z == x ∘ (y ∘ z)
        let lhs = xs.combine(&ys).combine(&zs);
        let rhs = xs.combine(&ys.combine(&zs));
        prop_assert_eq!(lhs, rhs);
    }
}

fn main() {}
