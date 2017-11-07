// todo get a shuffled iter going with RangeTo etc
extern crate rand;
use shuffled_iter::ShuffledIterGen;
use std::iter;
// use rand::Rng;

pub fn scratch() {
    for word in &["a", "b", "c"] {
        println!("{:?}", word);
    }

    for word in ["d", "e", "f"].iter() {
        println!("{:?}", word);
    }

    let mut rng = rand::XorShiftRng::new_unseeded();
    for x in rng.iter_shuffled(0..6) {
        println!("{:?}", x);
    }


        for x in rng.iter_shuffled(0..6) {
            println!("{:?}", x);
        }

    /* won't compile
    let words = &["a", "b", "c"];
    let si = ShuffledIter::new(10, words);
    for word in words.iter_shuffled() {
        println!("{:?}", word);
    }
    */
}
