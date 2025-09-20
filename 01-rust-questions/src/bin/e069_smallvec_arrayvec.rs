// 69. How do you use SmallVec or ArrayVec to collect small results without heap allocation for up to N items?
// What are the trade-offs of stack vs heap allocation?
use arrayvec::ArrayVec;
use smallvec::SmallVec;

fn main() {
    // Stack buffer: up to 4 elements
    let mut small: SmallVec<[i32; 4]> = SmallVec::new();
    let mut arr: ArrayVec<i32, 4> = ArrayVec::new();

    for i in 1..6 {
        small.push(i);
        println!(
            "Added {}, smallvec len={}, capacity={}",
            i,
            small.len(),
            small.capacity()
        );
    }
    for i in 1..5 {
        arr.push(i);
        println!("Added {}, ArrayVec len={}", i, arr.len());
    }

    println!("Final SmallVec: {:?}", small);
    println!("Final ArrayVec: {:?}", arr);
}
