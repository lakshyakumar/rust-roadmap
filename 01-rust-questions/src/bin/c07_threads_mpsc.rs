// Do not communicate by sharing memory instead share memory by communicating

use std::sync::mpsc;
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // single message
    // spawn(move || {
    //     let msg = String::from("hi");
    //     tx.send(msg).unwrap();
    //     // moved msg
    //     // println!("{}", msg);
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    //------------------------------------------------------

    // multiple messages
    // spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {}", received);
    // }

    //------------------------------------------------------

    // multiple Producers
    let tx2 = tx.clone();
    spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            sleep(Duration::from_secs(1));
        }
    });

    spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
