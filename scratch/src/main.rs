extern crate rand;
extern crate rayon;
use rand::Rng;
use rayon::prelude::*;
use std::{time, thread};

fn mine_nonce(payload: &Vec<u8>, nonce: u32) -> Option<String> {
    let millis = rand::thread_rng().gen_range(500, 900);
    let is_golden = rand::random::<f32>() >= 0.99;
    println!("N: {:>10} G: {:>5} S: {}", nonce, is_golden, millis);
    thread::sleep(time::Duration::from_millis(millis));
    if is_golden {
        Some(format!("block:{}", nonce))
    } else {
        None
    }
}

fn main() {
    let mut payload = vec![3, 4];
    let block = (0u32..u32::max_value())
        .into_par_iter()
        .map(|n| mine_nonce(&payload, n))
        .find_any(|o| o.is_some())
        .unwrap();
    println!("{:?}", block);
}
