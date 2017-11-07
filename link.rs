// Link to `library`, import items under the `lib_file` module
extern crate lib_file;

fn main() {
    lib_file::public_function();

    // Error! `private_function` is private
    //lib_file::private_function();

    lib_file::indirect_access();
}
