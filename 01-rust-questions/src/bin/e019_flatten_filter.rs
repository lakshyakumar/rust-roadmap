// 19. How can you flatten a Vec<Vec<i32>> and filter its elements using iterator chaining (into_iter().flatten().filter().collect())?
// Discuss the power of iterator combinators for data transformation.
fn main() {
    let data: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    // Flatten Vec<Vec<i32>> into Vec<i32>, then filter even numbers
    let evens: Vec<i32> = data
        .into_iter() // consume outer Vec
        .flatten() // turn Vec<Vec<i32>> into flat stream of i32
        .filter(|x| x % 2 == 0) // keep only evens
        .collect(); // collect into Vec<i32>

    println!("Flattened evens: {:?}", evens);
}
