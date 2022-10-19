#![allow(dead_code)]
fn main() {}

pub struct AveragedCollection {
    list: Vec<i32>,
    total: i32,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.total += value;
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.total -= value;
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn update_average(&mut self) {
        self.average = self.total as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: f64,
    pub height: f64,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Print the Button")
    }
}

pub struct SelectBox {
    width: f64,
    height: f64,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Print the SelectBox")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut avg = AveragedCollection {
            list: vec![],
            total: 0,
            average: 0f64,
        };
        for i in 10..123 {
            avg.add(i);
            println!("avg: {}", avg.average());
        }
    }

    #[test]
    fn test_2() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75f64,
                    height: 60f64,
                    options: vec!["Yes".to_string(), "MayBe".to_string(), "No".to_string()],
                }),
                Box::new(Button {
                    width: 75f64,
                    height: 60f64,
                    label: "OK".to_string(),
                }),
            ],
        };

        screen.run()
    }

    trait State {
        fn add_text(&self, _: &mut String, _: &str) {}

        fn request_review(self: Box<Self>) -> Box<dyn State>;

        fn approve(self: Box<Self>) -> Box<dyn State>;

        fn content<'a>(&self, _: &'a Post) -> &'a str {
            ""
        }

        fn reject(self: Box<Self>) -> Box<dyn State>;
    }

    pub struct Post {
        content: String,
        state: Option<Box<dyn State>>,
    }

    pub struct Draft {}
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn add_text(&self, content: &mut String, text: &str) {
            content.push_str(text);
        }
    }

    pub struct PendingReview {}
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }

    pub struct Published {}
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                content: String::new(),
                state: Option::Some(Box::new(Draft {})),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.state
                .as_ref()
                .unwrap()
                .add_text(&mut self.content, text)
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(&self)
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }
    }

    #[test]
    fn test_3() {
        let mut post = Post::new();

        // Draft
        post.add_text("Hello, world!");
        assert_eq!(post.content(), "");

        // Draft => PendingReview
        post.request_review();
        assert_eq!(post.content(), "");

        // PendingReview
        post.add_text("Hello, world!");
        assert_eq!(post.content(), "");

        // PendingReview => Draft
        post.reject();
        assert_eq!(post.content(), "");

        // Draft
        post.add_text("Hello, world!");
        assert_eq!(post.content(), "");

        // Draft => PendingReview
        post.request_review();
        assert_eq!(post.content(), "");

        // PendingReview => Published
        post.approve();
        assert_eq!(post.content(), "Hello, world!Hello, world!");
    }
}
