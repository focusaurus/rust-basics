extern crate rsfs;
use rsfs::{GenFS,FileType,Metadata};
use std::io::{Read, Seek, SeekFrom, Write};

fn main() {
    let fs = rsfs::mem::unix::FS::new();
    // fs.create_dir("/").expect("oops");
    // fs.set_current_dir("/").expect("oops");
    // let _empty = fs.create_file("/empty").unwrap();
    // , []).expect("should be writeable");
    let mut bytes = fs.create_file("/bytes").expect("should be writeable");
    bytes.write(b"inside of bytes").unwrap();
    // let contents = fs.metadata("/empty").expect("hey");
    // println!("{:?}", contents);
    let bytes_meta = fs.metadata("/bytes").unwrap();
    println!("{:?}", bytes_meta);
    println!("is_dir{:?}", bytes_meta.is_dir());
}
