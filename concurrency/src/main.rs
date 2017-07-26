use std::thread;

fn main() {
    thread::spawn(|| for i in 1..10 {
                      println!("hi number {:?} from spawned thread", i);
                  });
    for i in 1..5 {
        println!("main {}", i);
    }
}
