struct ImportantExceprt<'a> {
    part: &'a str,
}
// force git commit 3
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        return a;
    }
    return b;
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("oops");
    let excerpt = ImportantExceprt { part: first_sentence };
}
