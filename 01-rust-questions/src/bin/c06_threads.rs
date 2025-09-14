// btop on linux for memory and cpu utilization, resmon on win
use std::thread::spawn;
fn test_main() {
    let mut x = 0u128;
    for i in 1..500_000 {
        x += i;
    }
}

fn spawn_thread() -> std::thread::JoinHandle<()> {
    let thread_fn = || {
        let mut x = 0u128;
        for i in 1..500_000_000 {
            x += i;
        }
        println!("x: {}", x);
    };

    spawn(thread_fn)
}
fn main() {
    // running on main thread
    // test_main();

    //------------------------------------------------------------------------------------------------

    // spawning thread
    // let handle = spawn_thread();
    // handle.join().unwrap();

    //------------------------------------------------------------------------------------------------

    // spawning multiple threads
    // let handle1 = spawn_thread();
    // let handle2 = spawn_thread();
    // test_main();
    // println!("main thread completed");

    // let _ = handle1.join();
    // let _ = handle2.join();

    //------------------------------------------------------------------------------------------------
    // check if handle 1 or hand 2 was finished
    let handle1 = spawn_thread();
    let handle2 = spawn_thread();

    println!("main thread completed");
    loop {
        test_main();
        if handle1.is_finished() && handle2.is_finished() {
            println!("All threads completed");
            break;
        } else {
            println!("Lets check on after some time|")
        }
    }
}
