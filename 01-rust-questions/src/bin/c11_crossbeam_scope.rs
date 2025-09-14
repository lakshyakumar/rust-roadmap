// The Problem with Standard thread::spawn
//the spawned thread might outlive the function it was created in

use crossbeam::thread;

// fn wont_compile() {
//     let v = vec![1, 2, 3];
//     thread::spawn( || {
//         println!("{:?}", v);
//     });
// }

fn will_compile() {
    let v = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|_| {
            println!("Thread sees {:?}", v);
        });
        s.spawn(|_| {
            println!("Another thread sees len = {}", v.len());
        });
    })
    .unwrap();
}

fn main() {
    will_compile();
}
