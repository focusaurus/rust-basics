pub struct Block {
    pub nonce: u32,
    pub hash: String,
}

pub struct Answer {
    pub nonce: u32,
    pub block: Option<Block>,
}

pub fn to_bytes(nonce: u32) -> [u8; 4] {
    let mut bytes = [0u8; 4];
    let mut shifty = nonce;
    for i in 0..4 {
        bytes[i] = (shifty & 0b11111111) as u8;
        shifty = shifty >> 8;
    }
    bytes.reverse();
    bytes
}

pub fn leading_zero_bits(bytes: &[u8]) -> u8 {
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
