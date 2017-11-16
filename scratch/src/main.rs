fn main() {
    let mut payload = "abcd".to_string().into_bytes();
    let nonce_bytes = &"zzzz".to_string().into_bytes();
    let len = payload.len();

    let range = len-4..len;
    println!("range {:?}", range);
    payload.extend(nonce_bytes.iter());
    println!("p+n {:?}", payload);
    payload[len..len+4].copy_from_slice(&"yyyy".to_string().into_bytes());
    println!("p+n {:?}", payload);
}
