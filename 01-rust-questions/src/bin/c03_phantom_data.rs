// Compile time constraints for type safety
use std::marker::PhantomData;

#[derive(Debug)]
struct MyStruct<T> {
    f1: T,
    f2: PhantomData<T>,
}

fn main() {
    let ms = MyStruct {
        f1: 10,
        f2: PhantomData,
    };
    println!("{:?}", ms);
}
