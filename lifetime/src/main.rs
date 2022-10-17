#![allow(dead_code)]
use std::fmt::Display;

fn main() {
    let str1 = String::from("Hello World!");
    let str2;
    let result;
    {
        str2 = String::from("World");
    }
    result = longest(str1.as_str(), str2.as_str());
    println!("{}", result);

    println!("{}", first(str1.as_str(), str2.as_str()));

    let novel = String::from("Hello World!.Hello China");
    let first_sentence;
    let i;
    {
        first_sentence = novel.split('.').next().expect("Invalid sentence");
        i = ImportantExcerpt { name: first_sentence };
    }
    println!("{}", i.name);

    println!("{}", longest_with_announcement(str1.as_str(), str2.as_str(), "This is an announcement!"));
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

struct ImportantExcerpt<'a> {
    name: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_name(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self.name
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, announcement: &'a T) -> &'a str
    where T: Display + ?Sized,
{
    println!("Announcement: {}", announcement);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}