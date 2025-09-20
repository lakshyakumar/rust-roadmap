// 71. How do you implement a fixed-capacity MPMC ring buffer using atomics?
// When would you fall back to a Mutex if stuck? What are the challenges of lock-free buffers?
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const CAPACITY: usize = 8;

struct RingBuffer<T: Copy + Default> {
    buffer: Mutex<[T; CAPACITY]>,
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl<T: Copy + Default> RingBuffer<T> {
    fn new() -> Self {
        Self {
            buffer: Mutex::new([T::default(); CAPACITY]),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    fn push(&self, value: T) -> Result<(), T> {
        let mut head;
        loop {
            head = self.head.load(Ordering::Relaxed);
            let tail = self.tail.load(Ordering::Acquire);
            if (head + 1) % CAPACITY == tail {
                // buffer full
                return Err(value);
            }
            if self
                .head
                .compare_exchange_weak(
                    head,
                    (head + 1) % CAPACITY,
                    Ordering::Release,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                break;
            }
        }
        {
            let mut buf = self.buffer.lock().unwrap();
            buf[head] = value;
        }
        Ok(())
    }

    fn pop(&self) -> Option<T> {
        let mut tail;
        loop {
            tail = self.tail.load(Ordering::Relaxed);
            let head = self.head.load(Ordering::Acquire);
            if tail == head {
                // buffer empty
                return None;
            }
            if self
                .tail
                .compare_exchange_weak(
                    tail,
                    (tail + 1) % CAPACITY,
                    Ordering::Release,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                let buf = self.buffer.lock().unwrap();
                return Some(buf[tail]);
            }
        }
    }
}

fn main() {
    let buf = Arc::new(RingBuffer::<i32>::new());
    let buf2 = buf.clone();

    // Producer
    let producer = thread::spawn(move || {
        for i in 0..20 {
            loop {
                if buf.push(i).is_ok() {
                    println!("Produced: {}", i);
                    break;
                }
                thread::yield_now(); // spin or could fallback to Mutex
            }
        }
    });

    // Consumer
    let consumer = thread::spawn(move || {
        for _ in 0..20 {
            loop {
                if let Some(val) = buf2.pop() {
                    println!("Consumed: {}", val);
                    break;
                }
                thread::yield_now();
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
