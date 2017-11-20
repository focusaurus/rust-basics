extern crate filesystem;
use filesystem::FileSystem;

fn doit<F>(fs: &F, path: &str) -> String
    where F: FileSystem
{
    let contents = fs.read_file_to_string(path).expect("hey");
    return contents;
}

fn main() {
    let fs = filesystem::FakeFileSystem::default();
    let fs = filesystem::OsFileSystem::new();
    fs.create_dir_all("/tmp/scratch").expect("oops");
    fs.set_current_dir("/tmp/scratch").expect("oops");
    fs.write_file("empty", []).expect("should be writeable");
    fs.write_file("bytes", "inside of bytes".bytes().collect::<Vec<u8>>())
        .expect("should be writeable");
    println!("{:?}", doit(&fs, "empty").len());
    println!("{:?}", doit(&fs, "bytes"));
}
