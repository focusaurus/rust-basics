extern crate byte_sha;
extern crate hex_slice;
use hex_slice::AsHex;
use std::env;
use std::io::{self, Write};

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

fn leading_zero_bits(bytes: [u8; 4]) -> u32 {
    let mut zero_bit_count = 0;
    for orig_byte in &bytes {
        let mut byte = orig_byte.clone();
        // check each of the 8 bits in the byte
        for _bit in 0..8 {
            // check leftmost bit for zeroness
            if (byte & 128) == 0 {
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

fn main() {
    // get some bytes to represent the block data
    let block_string: String = env::args().skip(1).next().unwrap();
    let mut block_vec = block_string.into_bytes();

    // add 4 bytes space at the end for the nonce
    block_vec.append(&mut vec![0, 0, 0, 0]);
    let last_4_index = block_vec.len() - 4;
    let mut nonce = 0u32;
    loop {
        let nonce_bytes = to_bytes(nonce);
        // combine block data and nonce into a single slice
        block_vec[last_4_index] = nonce_bytes[0];
        block_vec[last_4_index + 1] = nonce_bytes[1];
        block_vec[last_4_index + 2] = nonce_bytes[2];
        block_vec[last_4_index + 3] = nonce_bytes[3];

        // compute checksum
        let hash: Vec<u8> = *byte_sha::sha256_of_message_as_u8_vec(&mut block_vec.clone());

        // check for magic success prefix
        if leading_zero_bits([hash[0], hash[1], hash[2], hash[3]]) >= 25 {
            println!("MINED! {:02x} with nonce {}", hash.as_hex(), nonce);
            break;
        } else {
            // if not, increment nonce and loop back around
            if nonce % 1_000_000 == 0 {
                io::stdout()
                    .write(b".")
                    .expect("Error writing to stdout");
                io::stdout().flush().expect("Error flushing stdout");
            }
            nonce += 1;
        }
    }
}
