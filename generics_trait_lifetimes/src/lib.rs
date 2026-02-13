use std::fmt::{Debug, Display};

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

impl NewsArticle {
    pub fn new(headline: &str, location: &str, author: &str, content: &str) -> Self {
        Self {
            headline: headline.to_string(),
            location: location.to_string(),
            author: author.to_string(),
            content: content.to_string(),
        }
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

impl Tweet {
    pub fn new(username: &str, content: &str, reply: bool, retweet: bool) -> Self {
        Self {
            username: username.to_string(),
            content: content.to_string(),
            reply,
            retweet,
        }
    }
}

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Pair<T>
where
    T: PartialOrd + Display,
{
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

#[derive(Debug)]
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}
