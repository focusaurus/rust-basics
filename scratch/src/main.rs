use std::thread;

fn main() {
    let input = "12 34 56 78"; // stack, str slice
    let char_vec = input
        .chars() // stack, lazy iterator over chars
        .filter(|c| c.is_numeric()) // stack, intermediate code/supporting data
        .collect::<Vec<char>>(); // stack, we now have a copy of a subset of `input`
    for (i, chunk) in char_vec.chunks(input.len() / 4).enumerate() {
        // About to throw a var into a move closure in a child thread
        // compiler can't check that parent will outlive child (?)
        // and also maybe data race checks are hard?
        // so we have to make this a move not a borrow
        let you_gotta_move_this_your_doing_fine = chunk.to_owned();
        thread::spawn(move || -> () {
                          println!("{:?}", you_gotta_move_this_your_doing_fine);
                      });
    }
}
