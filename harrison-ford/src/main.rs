fn main() {
    let mut actions = "Mustage Escape Escort Falcon Fiesta Focus"
        .split_whitespace()
        .collect::<Vec<_>>();
    // let mut actions = vec!["Mustang", "Escape", "Escort", "Falcon", "Fiesta", "Focus"];
    actions.sort();
    for action in actions {
        println!("Ford {}", action);
    }
}
