// 40. How do you use the itertools crate's group_by to group consecutive equal items in an iterator,
// avoiding manual state management? What are the benefits of using external crates for iterator utilities?
use itertools::Itertools;

fn main() {
    let data = vec![1, 1, 2, 2, 2, 3, 4, 4, 5];

    // Group consecutive equal items by their value
    let groups = data.into_iter().group_by(|&x| x);

    for (key, group) in &groups {
        let group_items: Vec<_> = group.collect();
        println!("Key: {}, Group: {:?}", key, group_items);
    }
}
