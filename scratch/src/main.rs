use std::io;

#[derive(Debug)]
struct Block {
    payload: Vec<u8>,
    nonce: u32,
    hash: String,
}

fn get_payload<'a>() -> Vec<u8> {
    "some data from greet_alloc_and_move".to_string().into_bytes()
}

fn mine<W: io::Write>(mut out: W, mut payload: Vec<u8>) -> io::Result<Block> {
    Ok(Block {
        nonce: 42,
        payload: payload,
        hash: "42".to_string(),
    })
}

fn main() {
    let payload = get_payload();
    let block = mine(io::stdout(), payload);
    println!("block {:?}", block.unwrap());
}
