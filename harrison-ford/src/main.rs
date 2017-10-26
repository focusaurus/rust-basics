use std::collections::BTreeSet;

const ACTIONS: &str = "Mustage Escape Escort Falcon Fiesta Focus";
fn main() {
    for action in ACTIONS.split_whitespace().collect::<BTreeSet<_>>() {
        println!("Ford {}", action);
    }
}
