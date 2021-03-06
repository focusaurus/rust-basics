//https://gist.github.com/gabrfarina/90ad75327362e47d7660f2fc6c25cd8a
trait Sorted {
    fn sorted(mut self) -> Self;
}

impl<T> Sorted for Vec<T>
    where T: Ord
{
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}
const ACTIONS: &str = "Mustage Escape Escort Falcon Fiesta Focus";

fn main() {
    for action in ACTIONS.split_whitespace().collect::<Vec<_>>().sorted(){
        println!("Ford {}", action);
    }
}

/*
use std::collections::BTreeSet;
const ACTIONS: &str = "Mustage Escape Escort Falcon Fiesta Focus";
fn main() {
    for action in ACTIONS.split_whitespace().collect::<BTreeSet<_>>() {
        println!("Ford {}", action);
    }
}
*/
