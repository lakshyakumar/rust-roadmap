// 25. How do you use Rayon parallel iterators to compute a parallel sum and histogram? Ensure no data races by using reduce_with.
// Why is Rayon preferred for data parallelism?
use rayon::prelude::*;
use std::collections::HashMap;

fn parallel_sum(data: &[i64]) -> i64 {
    data.par_iter() // parallel iterator over &i64
        .cloned() // turn &i64 → i64
        .reduce(|| 0, |a, b| a + b)
}

fn parallel_histogram(data: &[u8]) -> HashMap<u8, usize> {
    data.par_iter()
        .cloned()
        .map(|val| {
            let mut map = HashMap::new();
            *map.entry(val).or_insert(0) += 1;
            map
        })
        .reduce_with(|mut a, b| {
            // merge two partial histograms
            for (k, v) in b {
                *a.entry(k).or_insert(0) += v;
            }
            a
        })
        .unwrap_or_default()
}

fn main() {
    let nums: Vec<i64> = (1..=1_000_000).collect();
    let sum = parallel_sum(&nums);
    println!("Parallel sum = {}", sum);

    let data: Vec<u8> = (0..1000).map(|x| (x % 10) as u8).collect();
    let hist = parallel_histogram(&data);

    println!("Histogram: {:?}", hist);
}
