extern crate rayon;
extern crate rustblock;
extern crate shaman;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
use rustblock::{Answer, Block, to_bytes, leading_zero_bits};
use shaman::digest::Digest;
use std::io::{self, Write};
use std::time::SystemTime;

const DIFFICULTY: u8 = 30;

fn block_bytes() -> Vec<u8> {
    // get some bytes to represent the block data
    std::env::args()
        .skip(1)
        .next()
        .unwrap_or("sample input".to_string())
        .into_bytes()

}

fn mine_nonce(payload: &Vec<u8>, nonce: u32) -> Answer {
    // compute checksum
    let mut hasher = shaman::sha2::Sha256::new();
    hasher.input(&payload);
    hasher.input(&to_bytes(nonce));
    let mut hash = [0u8; 32];
    hasher.result(&mut hash);

    // check for magic success prefix ("golden nonce")
    if leading_zero_bits(&hash) >= DIFFICULTY {
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

fn mine_par(payload: &Vec<u8>) -> io::Result<Block> {
    let answer = (0u32..u32::max_value())
        .into_par_iter()
        .map(|nonce| mine_nonce(&payload, nonce))
        .find_any(|a| {
            if a.block.is_none() {
                if a.nonce % 1_000_000 == 0 {
                    // print!("."); // No good, buffered
                    let mut out = io::stdout();
                    out.write(b".").expect("Error during write");
                    out.flush().expect("Error during flush");
                    // let mut handle = out.lock();
                    // handle.write(b".").expect("Error during write");
                    // handle.flush().expect("Error during flush");
                }
            }
            a.block.is_some()
        })
        .unwrap();
    Ok(answer.block.unwrap())
}

fn main() {
    let now = SystemTime::now();
    match mine_par(&block_bytes()) {
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(10);
        }
        Ok(block) => {

            println!("
MINED!
hash: {}
difficulty: {}
nonce: {}
time: {} seconds
",
                     block.hash,
                     DIFFICULTY,
                     block.nonce,
                     now.elapsed().unwrap().as_secs());
        }
    }

}
