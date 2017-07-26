use std::thread;

fn main() {
    let handle = thread::spawn(|| for i in 1..10 {
                                   println!("hi number {:?} from spawned thread", i);
                               });
    handle.join();
    for i in 1..5 {
        println!("main {}", i);
    }
}
