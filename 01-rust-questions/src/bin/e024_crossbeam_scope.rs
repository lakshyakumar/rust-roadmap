// 24. Compare parallel mapping over a slice using crossbeam scoped threads versus using Rayon for large data sets.
// What are the trade-offs in terms of ergonomics and performance?

use crossbeam::scope;

fn parallel_map_crossbeam(data: &mut [i32]) {
    let n_threads = 4;
    let chunk_size = (data.len() + n_threads - 1) / n_threads;

    scope(|s| {
        for chunk in data.chunks_mut(chunk_size) {
            s.spawn(move |_| {
                for x in chunk {
                    *x *= 2; // example: multiply by 2
                }
            });
        }
    })
    .unwrap();
}

fn main() {
    let mut nums: Vec<i32> = (0..20).collect();
    parallel_map_crossbeam(&mut nums);
    println!("Crossbeam result: {:?}", nums);
}
