extern crate shaman;
use shaman::digest::Digest;
use std::env;
use std::io::{self, Write};

const DIFFICULTY: u8 = 24;

fn to_bytes(nonce: u32) -> [u8; 4] {
    let mut bytes = [0u8; 4];
    let mut shifty = nonce;
    for i in 0..4 {
        bytes[i] = (shifty & 255) as u8;
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
            if (byte & 128) == 0 {
                // 128 in binary is 10000000
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

fn mine() -> io::Result<(String,u32)> {
    // get some bytes to represent the block data
    let block_string: String = env::args()
        .skip(1)
        .next()
        .unwrap_or("sample input".to_string());
    let mut block_vec = block_string.into_bytes();

    // add 4 bytes space at the end for the nonce
    block_vec.append(&mut vec![0, 0, 0, 0]);
    let last_4_index = block_vec.len() - 4;
    let mut nonce: u32 = 0;
    let mut hasher = shaman::sha2::Sha256::new();

    loop {
        let nonce_bytes = to_bytes(nonce);
        // combine block data and nonce into a single slice
        for i in 0..4 {
            block_vec[last_4_index + i] = nonce_bytes[i];
        }

        // compute checksum
        hasher.reset();
        hasher.input(&block_vec);
        let mut hash = [0u8; 32];
        hasher.result(&mut hash);

        // check for magic success prefix ("golden nonce")
        if leading_zero_bits(&hash[0..4]) < DIFFICULTY {
            // nope, increment nonce and loop back around
            if nonce % 1_000_000 == 0 {
                io::stdout().write(b".")?;
                io::stdout().flush()?;
            }
            nonce += 1;
            continue;
        }
        break;
    }
    Ok((hasher.result_str(), nonce))
}

fn main() {
    match mine() {
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(10);
        }
        Ok((hash, nonce)) => {
           println!("MINED! {} with nonce {}", hash, nonce);
        }
    }
}
