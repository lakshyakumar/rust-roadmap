// 84. How do you implement a basic work-stealing thread pool using crossbeam-deque?
// Run tasks with futures::executor::block_on via a spawn_blocking equivalent. Why is work-stealing useful?

// low-level concurrency: implementing a tiny work-stealing thread pool using crossbeam-deque
//  and driving tasks manually.

// 🔹 Why Work-Stealing?
// A naive thread pool assigns tasks round-robin. If one thread gets a heavy task, it stays busy while others are idle.

// Work-stealing solves this:
// Each worker thread has its own deque (double-ended queue).
// Workers push/pop tasks from the bottom (LIFO, good for cache locality).
// If a worker runs out of work, it steals from the top of another worker’s deque.
// ✅ Leads to better load balancing, lower idle time, and higher throughput.

use crossbeam_deque::{Injector, Steal, Stealer, Worker};
use futures::executor::block_on;
use std::sync::Arc;
use std::thread;

type Task = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    injector: Arc<Injector<Task>>,
    stealers: Vec<Stealer<Task>>,
    workers: Vec<Worker<Task>>,
}

impl ThreadPool {
    fn new(num_threads: usize) -> Self {
        let injector = Arc::new(Injector::new());
        let mut workers = Vec::new();
        let mut stealers = Vec::new();

        for _ in 0..num_threads {
            let worker = Worker::new_fifo(); // FIFO for fairness, LIFO also possible
            stealers.push(worker.stealer());
            workers.push(worker);
        }

        Self {
            injector,
            stealers,
            workers,
        }
    }

    fn spawn(&self, task: Task) {
        self.injector.push(task);
    }

    fn run(&mut self) {
        let injector = self.injector.clone();
        let stealers = self.stealers.clone();
        let mut workers = std::mem::take(&mut self.workers);

        let threads: Vec<_> = workers
            .into_iter()
            .map(|local_worker| {
                let injector = injector.clone();
                let stealers = stealers.clone();

                thread::spawn(move || {
                    loop {
                        let task = local_worker
                            .pop()
                            .or_else(|| injector.steal().success())
                            .or_else(|| stealers.iter().filter_map(|s| s.steal().success()).next());

                        match task {
                            Some(task) => (task)(),
                            None => break, // exit when no more work
                        }
                    }
                })
            })
            .collect();

        for th in threads {
            th.join().unwrap();
        }
    }
}

// ---- Example usage ----
fn main() {
    let mut pool = ThreadPool::new(4);

    for i in 0..10 {
        pool.spawn(Box::new(move || {
            block_on(async {
                println!("Task {i} running on thread {:?}", thread::current().id());
            });
        }));
    }

    pool.run();
    println!("All tasks complete");
}
