extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();
    let salad = String::from("I ate a salad for lunch today.");
    post.add_text(&salad);
    let post = post.request_review();
    let post = post.approve();
    assert_eq!(salad, post.content());

}
