#![allow(dead_code)]
use std::ops::Add;

fn main() {
    let s = String::from("Hello").add(" World");
    let mut s1 = take_ownership_then_return(s); // s move 到了函数中，从现在开始不再有效
    println!("{}", s1);

    let _re = &s1;
    let _r2 = &s1;
    let _r3 = &mut s1;
    let len = calculate_len(_r3);
    println!("s1:{} len = {}", s1, len);
    // println!("s1:{} len = {}", _re, len); 这里会报错

    let i: u32 = 1;
    move_copy(i);
    println!("{}", i); // u32 是Copy的，所以这里i依旧有效

    no_dangle();

    // 切片 slice
    let s = String::from("Hello World");
    let hello = &s[..5];
    let world = &s[6..11];
    println!("{}", hello);
    println!("{}", world);

    // take_ownership("Hello"); 这里是错误的，因为字符串常量其实是字符串切片, 而函数需要一个字符串

    println!("First world is {}", first_world(&s));
}

fn take_ownership(s: String) {
    println!("{}", s)
}

fn take_ownership_then_return(s: String) -> String {
    s
}

fn move_copy(_: u32) {}

fn calculate_len(s: &mut String) -> usize { // & 引用，表示当前函数不获取s的所有权，而是使用s的引用, 也被称为借用(borrowing)
    s.push_str("fd");
    s.len()
}

// 下面这个函数会发生悬垂引用
// fn dangle() -> &String {
//     let s = String::from("HI");
//     &s
// }
fn no_dangle() -> String { // 不是引用的话, 这里会传递所有权
    let s = String::from("HI");
    s
}

fn first_world(s: &str) -> &str { // &str 表示字符切片
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
