use generics_trait_lifetimes::{NewsArticle,ImportantExcerpt, Summary, Tweet};
use std::fmt::{Debug, Display};

fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// fn some_function<T: Display + Clone, U: Clone + Debug>(x: T) -> T {
//     x
// }
fn some_function<T, U>(x: T) -> T
where
    T: Display + Clone,
    U: Clone + Debug,
{
    x
}
fn some_function1(x: impl Display + Clone) -> i32 {
    1
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn largest_item<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let string_list = vec!["one", "two", "three", "four", "five"];
    let new_article = NewsArticle::new("New York Times", "New York", "author", "content");
    println!("{}", new_article.summarize());
    let new_tweet = Tweet::new("username", "content", false, false);
    println!("{}", new_tweet.summarize());
    notify(&new_article);
    notify(&new_tweet);
    println!("{}", largest_item(&number_list));
    println!("{}", largest_item(&string_list));
    println!("{:#?}", number_list);
    println!("{:#?}", string_list);
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:#?}", i);
}
