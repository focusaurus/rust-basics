// use rand::Rng;
// use rayon::prelude::*;
extern crate rand;
extern crate shaman;
use shaman::digest::Digest;
use std::{env,time, thread};
use std::io::{self,Write};
extern crate sha2;
extern crate digest;
extern crate digest_writer;
// use std::fs::File;
// use std::io::{self, Write};
// use digest::FixedOutput;
// use digest_writer::Writer;

fn main() {
    let payload = env::args()
        .skip(1)
        .nth(0)
        .unwrap()
        .to_string()
        .into_bytes();
        // let mut digest = Writer::new(sha2::Sha256::default());
        // let mut f = File::open("LICENSE-MIT").unwrap();
        // io::copy(&mut f, &mut digest).unwrap();
        // digest.fixed_result();
    let nonce = "aaaa".to_string().into_bytes();
    let mut hasher = shaman::sha2::Sha256::new();
    hasher.input(&payload);
    hasher.input(&nonce);
    let mut hash = [0u8; 32];
    hasher.result(&mut hash);
    println!("{:?}: {}", payload, hasher.result_str());
}
