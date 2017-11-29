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

pub fn leading_zero_bits(bytes: &[u8]) -> u32 {
    let mut zero_bit_count = 0;
    for byte in bytes {
        let zeros = byte.leading_zeros();
        zero_bit_count += zeros;
        if zeros < 8 {
            return zero_bit_count;
        }
    }
    zero_bit_count
}

#[test]
fn test_leading_zero_bits() {
    assert_eq!(leading_zero_bits(&[]), 0);
    assert_eq!(leading_zero_bits(&[0b11111111]), 0);
    assert_eq!(leading_zero_bits(&[0b00000111]), 5);
    assert_eq!(leading_zero_bits(&[0b00000000]), 8);
    assert_eq!(leading_zero_bits(&[0b00000000, 0b00100000]), 10);
    assert_eq!(leading_zero_bits(&[0b00000000, 0b00000000, 0b00000001]), 23);
    assert_eq!(leading_zero_bits(&[0b01000000, 0b00000000, 0b00000001]), 1);
}
