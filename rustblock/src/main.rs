extern crate rayon;
extern crate shaman;
use rayon::prelude::*;
use shaman::digest::Digest;
use std::{env, io};
use rayon::iter::IntoParallelIterator;
use std::io::Write;

const DIFFICULTY: u8 = 4;

struct Block {
    nonce: u32,
    hash: String,
    // payload: Vec<u8>,
}

fn to_bytes(nonce: u32) -> [u8; 4] {
    let mut bytes = [0u8; 4];
    let mut shifty = nonce;
    for i in 0..4 {
        bytes[i] = (shifty & 0b11111111) as u8;
        shifty = shifty >> 8;
    }
    bytes.reverse();
    bytes
}

fn leading_zero_bits(bytes: &[u8]) -> u8 {
    let mut zero_bit_count = 0;
    for orig_byte in bytes {
        let mut byte = orig_byte.clone();
        // check each of the 8 bits in the byte
        for _bit in 0..8 {
            // check leftmost bit for zeroness
            if (byte & 0b10000000) == 0 {
                zero_bit_count += 1;
                // left shift 1 bit to check the next bit
                byte = byte << 1;
            } else {
                return zero_bit_count;
            }
        }
    }
    zero_bit_count
}

fn block_bytes() -> Vec<u8> {
    // get some bytes to represent the block data
    env::args()
        .skip(1)
        .next()
        .unwrap_or("sample input".to_string())
        .into_bytes()

}

fn mine<W: io::Write>(mut out: W, mut payload: Vec<u8>) -> io::Result<Block> {
    let len = payload.len();
    // add 4 bytes space for the nonce at the end of the payload
    payload.extend([0, 0, 0, 0].iter());
    let mut nonce: u32 = 0;
    let mut hasher = shaman::sha2::Sha256::new();

    loop {
        let nonce_bytes = to_bytes(nonce);

        // combine block data and nonce into a single slice
        payload[len..len + 4].copy_from_slice(&nonce_bytes);

        // compute checksum
        hasher.reset();
        hasher.input(&payload);
        let mut hash = [0u8; 32];
        hasher.result(&mut hash);

        // check for magic success prefix ("golden nonce")
        if leading_zero_bits(&hash[0..4]) < DIFFICULTY {
            // nope, increment nonce and loop back around
            if nonce % 1_000_000 == 0 {
                out.write(b".")?;
                out.flush()?;
            }
            nonce += 1;
            continue;
        }
        break;
    }
    let block = Block {
        hash: hasher.result_str(),
        nonce,
        // payload,
    };
    Ok(block)
}

fn mine_nonce(mut payload: Vec<u8>, nonce: u32) -> Option<Block> {
    let len = payload.len();
    // add 4 bytes space for the nonce at the end of the payload
    payload.extend([0, 0, 0, 0].iter());
    let mut hasher = shaman::sha2::Sha256::new();
    let nonce_bytes = to_bytes(nonce);

    // combine block data and nonce into a single slice
    payload[len..len + 4].copy_from_slice(&nonce_bytes);

    // compute checksum
    hasher.reset();
    hasher.input(&payload);
    let mut hash = [0u8; 32];
    hasher.result(&mut hash);

    // check for magic success prefix ("golden nonce")
    if leading_zero_bits(&hash[0..4]) >= DIFFICULTY {
        let block = Block {
            hash: hasher.result_str(),
            nonce,
        };
        Some(block)
    } else {
        None
    }
}

fn main1() {
    // get some bytes to represent the block data
    let payload = block_bytes();
    match mine(io::stdout(), payload) {
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(10);
        }
        Ok(block) => {
            println!("MINED! {} with nonce {}", block.hash, block.nonce);
        }
    }

}


fn mine_par<W: io::Write>(mut out: W, mut payload: Vec<u8>) -> io::Result<Block> {
    let len = payload.len();
    // add 4 bytes space for the nonce at the end of the payload
    payload.extend([0, 0, 0, 0].iter());
    let block = (0u32..u32::max_value())
        .into_par_iter()
        // .map(|&nonce| {
        //          return mine_nonce(payload, nonce).or_else(|| if nonce % 1_000_000 == 0 {
        //                                                out.write(b".").expect("Error stdout");
        //                                                out.flush().expect("Error flushing stdout");
        //                                            });
        //      })
        .map(|nonce| mine_nonce(vec![0,1,2,3,4,5,6,7], nonce))
        // .find_any(|o| o.is_some() )
        .find_any(|o| {
            if o.is_none() {
                    let mut out = io::stdout();
                    out.write(b".").expect("w");
                    out.flush().expect("f");
            }
            o.is_some()
         })
        .unwrap();
    Ok(block.unwrap())
}

fn main() {
    // get some bytes to represent the block data
    let payload = block_bytes();
    let mut stdout = io::stdout();
    match mine_par(stdout, payload) {
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(10);
        }
        Ok(block) => {
            println!("MINED! {} with nonce {}", block.hash, block.nonce);
        }
    }

}
