extern crate filesystem;
use filesystem::FileSystem;


fn main() {
    let fs = filesystem::FakeFileSystem::default();
    fs.create_dir("/").expect("oops");
    fs.set_current_dir("/").expect("oops");
    fs.write_file("empty", []).expect("should be writeable");
    fs.write_file("bytes", "inside of bytes".bytes().collect::<Vec<u8>>())
        .expect("should be writeable");
    let contents = fs.read_file("empty").expect("hey");
    println!("{:?}", contents.len());
    println!("{:?}", fs.read_file_to_string("bytes").unwrap());
}
