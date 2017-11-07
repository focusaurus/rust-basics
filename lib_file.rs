pub fn public_function() {
    println!("called lib_file's `public_function()`");
}

fn private_function() {
    println!("called lib_file's `private_function()`");
}

pub fn indirect_access() {
    print!("called lib_file's `indirect_access()`, that\n> ");

    private_function();
}
