use std::thread;

fn do_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
                                   println!("vec from thread {:?}", v);
                               });
    handle.join();
    for i in 1..5 {
        println!("main {}", i);
    }
}
use std::sync::mpsc;
use std::time::Duration;
extern crate rand;
use rand::Rng;

fn do_channel() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![String::from("hi"),
                        String::from("from"),
                        String::from("the"),
                        String::from("thread")];
        for val in vals {
            tx.send(val).unwrap();
            let duration = rand::thread_rng().gen_range(0, 4);
            println!("Will sleep {:?}", duration);
            thread::sleep(Duration::from_secs(duration));
        }
    });
    thread::spawn(move || {
        let vals = vec![String::from("more"),
                        String::from("messages"),
                        String::from("2nd"),
                        String::from("thread")];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {:?}", received);
    }
}
fn main() {
    do_thread();
    do_channel();
}
