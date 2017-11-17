extern crate rayon;
extern crate rustblock;
extern crate shaman;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use rustblock::{Answer,Block,to_bytes,leading_zero_bits};
use shaman::digest::Digest;
use std::{env, io};
use std::io::Write;

const DIFFICULTY: u8 = 29;

fn block_bytes() -> Vec<u8> {
    // get some bytes to represent the block data
    env::args()
        .skip(1)
        .next()
        .unwrap_or("sample input".to_string())
        .into_bytes()

}

fn mine_nonce(payload: &Vec<u8>, nonce: u32) -> Answer {
    let len = payload.len();
    // // add 4 bytes space for the nonce at the end of the payload
    // payload.extend([0, 0, 0, 0].iter());
    let mut hasher = shaman::sha2::Sha256::new();
    // let nonce_bytes = to_bytes(nonce);

    // combine block data and nonce into a single slice
    // payload[len..len + 4].copy_from_slice(&nonce_bytes);

    // compute checksum
    hasher.reset();
    hasher.input(&payload);
    hasher.input(&to_bytes(nonce));
    let mut hash = [0u8; 32];
    hasher.result(&mut hash);

    // check for magic success prefix ("golden nonce")
    if leading_zero_bits(&hash[0..4]) >= DIFFICULTY {
        let block = Block {
            hash: hasher.result_str(),
            nonce,
        };
        Answer {
            nonce,
            block: Some(block),
        }
    } else {
        Answer { nonce, block: None }
    }
}

fn mine_par<W: io::Write>(mut out: W, payload: &Vec<u8>) -> io::Result<Block> {
    let answer = (0u32..u32::max_value())
        .into_par_iter()
        .map(|nonce| mine_nonce(&payload, nonce))
        .find_any(|a| {
            if a.block.is_none() {
                if a.nonce % 1_000_000 == 0 {
                    let mut out = io::stdout();
                    out.write(b".").expect("w");
                    out.flush().expect("f");
                }
            }
            a.block.is_some()
        }).unwrap();
    Ok(answer.block.unwrap())
}

fn main() {
    match mine_par(io::stdout(), &block_bytes()) {
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(10);
        }
        Ok(block) => {
            println!("MINED!\n{} with nonce {}", block.hash, block.nonce);
        }
    }

}
