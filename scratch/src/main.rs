extern crate rsfs;
extern crate pem;
use rsfs::GenFS;
use std::io::Read;

// ssh-keygen -f test-ssh-key-1 -t ed25519 -C "Tealeves test ED25519 SSH Key 1"
fn main() {
    let fs = rsfs::disk::FS;
    let mut file = fs.open_file("/tmp/test-ssh-ed25519-1.privatekey")
        .expect("expect1");
    let mut key_string = String::new();
    file.read_to_string(&mut key_string).unwrap();
    let parsed = pem::parse(key_string).unwrap();
    // let mut parts: Vec<&str> = key_string.lines().collect();
    // let footer = parts.pop().unwrap();
    // let payload: String = parts[1..].join("");
    // split_whitespace().collect();
    // let key_type = parts.get(0).unwrap();
    // let payload = parts.get(1).unwrap();
    // let comment = parts[2..].join(" ");
    // print!("{} {}", payload, footer);
    // println!("{:?}", parsed.contents[0..10].iter().collect::<String>());
    // println!("{}", String::new(parsed.contents[0..10].into()));
    let magic = "openssh-key-v1";
    let found = String::from_utf8(parsed.contents[0..magic.len()].to_vec()).unwrap();
    println!("{} {}", found, magic == found);
    let payload_hex: String = parsed.contents.iter().map(|byte| format!("{:x}", byte)).collect();
    println!("{}: {}", parsed.tag, payload_hex);
}
