use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
                                       let mut num = counter.lock().unwrap();

                                       *num += 1;
                                   });
        handles.push(handle);
    }
    // let handle = thread::spawn(move || {
    //                                let mut num = counter.lock().unwrap();
    //                                *num += 1;
    //                            });
    // handles.push(handle);
    // let handle2 = thread::spawn(move || {
    //                                 let mut num2 = counter.lock().unwrap();
    //                                 *num2 += 1;
    //                             });
    // handles.push(handle2);
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {:?}", *counter.lock().unwrap());
}
