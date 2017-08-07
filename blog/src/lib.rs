pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }
}
//
//     pub fn reject(&mut self) {
//         if let Some(state) = self.state.take() {
//             self.state = Some(state.reject())
//         }
//     }
//
//     pub fn approve(&mut self) {
//         if let Some(state) = self.state.take() {
//             self.state = Some(state.approve())
//         }
//     }
// }
//
// trait State {
//     fn request_review(self: Box<Self>) -> Box<State>;
//     fn approve(self: Box<Self>) -> Box<State>;
//     fn reject(self: Box<Self>) -> Box<State>;
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
// }
//
// struct Draft {}
//
// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<State> {
//         Box::new(PendingReview {})
//     }
//
//     fn approve(self: Box<Self>) -> Box<State> {
//         self
//     }
//     fn reject(self: Box<Self>) -> Box<State> {
//         self
//     }
// }
// struct PendingReview {}
//
// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<State> {
//         self
//     }
//
//     fn approve(self: Box<Self>) -> Box<State> {
//         Box::new(Published {})
//     }
//
//     fn reject(self: Box<Self>) -> Box<State> {
//         Box::new(Draft{})
//     }
// }
//
// struct Published {}
//
// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<State> {
//         self
//     }
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
//
//     fn reject(self: Box<Self>) -> Box<State> {
//         Box::new(Draft{})
//     }
// }
