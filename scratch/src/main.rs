extern crate rsfs;
use rsfs::*;
use rsfs::unix_ext::*;
use rsfs::unix_ext::{PermissionsExt, GenFSExt};

fn helper<F: GenFS<Permissions = rsfs::disk::Permissions>>(fs: &rsfs::disk::FS) {
    // fn helper(fs: &rsfs::disk::FS) {
    let meta = fs.metadata("/").unwrap();
    let perms = meta.permissions();
    // assert_eq!(perms.mode(), 0o755);
    println!("{:?}", perms.mode());
}

fn main() {
    let fs = rsfs::mem::unix::FS::new();
    fs.create_dir_all("/tmp")
    let file = fs.create_file("/tmp/empty");
    // let fs = rsfs::mem::unix::FS;
    // helper::<rsfs::mem::unix::FS>(&fs);
    helper2::<rsfs::mem::unix::Permissions, rsfs::mem::unix::Metadata, rsfs::mem::unix::FS>(&fs);
    helper2::<rsfs::disk::Permissions, rsfs::disk::Metadata, rsfs::disk::FS>(&rsfs::disk::FS);
    helper2(&rsfs::disk::FS);
    helper2(&fs);
}

fn helper2<P: Permissions + PermissionsExt,
           M: Metadata<Permissions = P>,
           F: GenFS<Permissions = P, Metadata = M>>
    (fs: &F) {
    let meta = fs.metadata("/").unwrap();
    // let perms = M::permissions(&meta);
    let perms = meta.permissions();
    // .permissions();
    // assert_eq!(perms.mode(), 0o755);
    println!("{:?}", perms.mode());
}
