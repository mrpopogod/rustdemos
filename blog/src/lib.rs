pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            // This combined with the Box<self> syntax on the request_review method
            // is why we need the Option; if we try to do a simple Box<State> and do
            // self.state = self.state.request_review() the ownership doesn't work out;
            // you get a compile error where it's trying to move other_state out of self
            // (to be taken by request_review) which isn't allowed.  Doing it this way
            // leaves the Option still attached to the Post, and then we can update it
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>; // this syntax scopes the method to only be eligible to be called
                                                          // on a Box containing this type; the method will take ownership
                                                          // of the Box in the process
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // can't approve a draft, so it'll stay a draft
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // if it's already in review asking for a review does nothing
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // published is out the door, so too late for workflow
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // published is out the door, so too late for workflow
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
