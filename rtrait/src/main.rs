pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "This is a tweet".to_string(),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());

    let news = NewsArticle {
        headline: "News".to_string(),
        location: "San Francisco, CA".to_string(),
        author: "Horse Ebooks".to_string(),
        content: "This is a news".to_string(),
    };
    notify(news);
}

