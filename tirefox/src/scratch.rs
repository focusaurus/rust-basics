// todo get a shuffled iter going with RangeTo etc
/*
extern crate rand;
// use shuffled_iter;
use shuffled_iter::ShuffledIterGen;

// use std::iter;
// use rand::Rng;
pub fn scratch() {
    for word in &["a", "b", "c"] {
        println!("{:?}", word);
    }

    for word in ["d", "e", "f"].iter() {
        println!("{:?}", word);
    }

    // let mut rng = rand::XorShiftRng::new_unseeded();
    // for x in rng.iter_shuffled(0..6) {
    //     println!("{:?}", x);
    // }

    let mut rng = rand::thread_rng();
    for x in rng.iter_shuffled(0u32..6u32).take(3) {
        // Type errors: try innermost thing annotating first
        println!("{:?}", x);
    }
    // let mut iter: shuffled_iter::ShuffledIter = rng.iter_shuffled(0..6).take::<u32>(3u32);
    /*
    let mut iter: shuffled_iter::ShuffledIter<Item = u32> = rng.iter_shuffled(0..6).take(3);
    for x in iter {
        println!("{:?}", x);
    }
*/










    /* won't compile
    let words = &["a", "b", "c"];
    let si = ShuffledIter::new(10, words);
    for word in words.iter_shuffled() {
        println!("{:?}", word);
    }
    */
}
*/

pub fn scratch() {
    let mut vec_a: Vec<&str> = vec![];
    vec_a.push("a");
    vec_a.push("b");
    vec_a.push("c");
    vec_a.push("d");
    let slice = &vec_a;
    for value in slice.chunks(2) {
        println!("{:?}", value);
    }
}
