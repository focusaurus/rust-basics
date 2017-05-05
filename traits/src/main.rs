pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("Read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub content: String,
    pub author: String,
}

impl Summarizable for NewsArticle {
    //     fn summary(&self) -> String {
    //         format!("{}, by {} ({})", self.headline, self.author, self.location)
    //     }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}
fn main() {
    let tweet = Tweet {
        username: "focusaurus".to_string(),
        content: "Hey traits tweet".to_string(),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Trump ruins everything"),
        author: String::from("John Doe"),
        location: String::from("Springfield"),
        content: String::from("Blah blah"),
    };
    println!("Traits: {}", tweet.summary());
    println!("Article: {}", article.summary());
}
