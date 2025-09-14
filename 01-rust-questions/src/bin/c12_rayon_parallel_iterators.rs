// Rayon is a data parallelism library.
use rayon::prelude::*;

fn main() {
    let v: Vec<i32> = (1..=10).collect();

    // Sequential
    let sum_seq: i32 = v.iter().map(|x| x * 2).sum();
    println!("Sequential sum is  {}.", sum_seq);

    // Parallel
    let sum_par: i32 = v.par_iter().map(|x| x * 2).sum();
    println!("Parallel sum = {}", sum_par);
}
