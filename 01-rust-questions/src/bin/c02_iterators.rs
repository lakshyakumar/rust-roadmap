// Iterators
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    // Iterator with for
    let numbers = vec![1, 2, 3, 4, 5];

    for num in numbers.iter() {
        println!("{:?}", num);
    }

    // consumeable adaptors
    // Iterator moves here
    let numbers = vec![1, 2, 3, 4, 5];
    let total: i32 = numbers.iter().sum();

    println!("The sum is : {}", total);

    // chained iterators

    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    let even: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).cloned().collect();

    println!("Even numbers: {:?}", even);
}
