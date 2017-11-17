extern crate rand;
extern crate rayon;
use rand::Rng;
use rayon::prelude::*;
use std::{time, thread};

// fn mine_nonce(mut payload: Vec<u8>, nonce: u32) -> Option<String> {
//
// }
fn biz(nonce: u32) -> Option<u32> {
    let millis = rand::thread_rng().gen_range(0.5, 3.5) * 1000.0;
    let chance = rand::random::<f32>();
    println!("checking {} sleep {} chance {}", nonce, millis, chance);
    thread::sleep(time::Duration::from_millis(millis as u64));
    if chance >= 0.5 { Some(nonce) } else {None}
}

fn main() {
    let nonce = (0u32..11).into_par_iter().find_any(|&i|biz(i).is_some());
    println!("{:?}", nonce);
}
