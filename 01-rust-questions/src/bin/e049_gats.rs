// 49. What are generic associated types (GATs)? Define a trait StreamingIterator with type Item<'a>, and implement it for a struct over a buffer.
// How do GATs improve trait expressiveness?

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
// Generic Associated Types (GATs) extend this concept by allowing the associated type to have its own generic parameters or lifetimes:

// trait StreamingIterator {
//     type Item<'a>; // GAT: Item depends on a lifetime 'a
// }

struct Buffer {
    data: Vec<u8>,
    pos: usize,
}

trait StreamingIterator {
    type Item<'a>
    where
        Self: 'a; // GAT: depends on a lifetime

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

impl StreamingIterator for Buffer {
    type Item<'a> = &'a u8; // Return reference to element in buffer

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.pos < self.data.len() {
            let item = &self.data[self.pos];
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}

fn main() {
    let mut buf = Buffer {
        data: vec![10, 20, 30],
        pos: 0,
    };

    while let Some(x) = buf.next() {
        println!("{}", x);
    }
}
