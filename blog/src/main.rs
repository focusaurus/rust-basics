extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();
    let salad = String::from("I ate a salad for lunch today.");
    post.add_text(&salad);
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!(salad, post.content());

}
