// 32. How do you implement exponential backoff and jitter for retries in async Rust?
// Write a function retry(f, attempts, base_delay). Why is backoff important for reliability?

use rand::{rng, Rng};
use std::time::Duration;
use tokio::time::sleep;

async fn retry<F, Fut, T, E>(mut f: F, attempts: usize, base_delay: Duration) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut delay = base_delay;
    for i in 0..attempts {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if i + 1 == attempts {
                    return Err(e);
                }

                // jitter between 0.5x and 1.5x delay
                let jitter: f64 = rng().random_range(0.5..1.5);
                let sleep_duration = delay.mul_f64(jitter);

                println!(
                    "Attempt {} failed, retrying in {:?}...",
                    i + 1,
                    sleep_duration
                );

                sleep(sleep_duration).await;
                delay *= 2;
            }
        }
    }
    unreachable!()
}

#[tokio::main]
async fn main() {
    let result: Result<&str, &str> = retry(
        || async {
            // Example "unstable task"
            if rand::random::<u8>() % 19 == 0 {
                Ok("Success!")
            } else {
                Err("Failed")
            }
        },
        5,
        Duration::from_millis(200),
    )
    .await;

    println!("Final result: {:?}", result);
}
