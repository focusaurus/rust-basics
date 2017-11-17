// use rand::Rng;
// use rayon::prelude::*;
extern crate rand;
extern crate shaman;
use shaman::digest::Digest;
use std::{env, io, time, thread};

fn main() {
    let payload = env::args()
        .skip(1)
        .nth(0)
        .unwrap()
        .to_string()
        .into_bytes();
    let nonce = "aaa".to_string().into_bytes();
    let mut hasher = shaman::sha2::Sha256::new();
    hasher.input(&payload);
    hasher.input(&nonce);
    let mut hash = [0u8; 32];
    hasher.result(&mut hash);
    println!("{:?}: {}", payload, hasher.result_str());
}
