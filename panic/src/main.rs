use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let _file = match File::open("hello.txt") {
        Ok(f) => f,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(f) => f,
                    Err(e) => panic!("Error creating file, {:?}", e),
                },
                other_error=> panic!("Error: {:?}", other_error),
            }
        },
    };

    let _f = File::open("hello.txt").unwrap();

    let _i = File::open("hello_three.txt").expect("Error: opening file");
}

fn read_user_name_from_file() -> Result<String, io::Error> {
    let f = File::open("hello_three.txt");

    let mut f = match f {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 使用?将错误返回给调用者
// 只能用于返回Result,Option或其他实现了Try类型的函数
fn read_user_name_from_file_simplify() -> Result<String, io::Error> {
    let mut f = File::open("hello_three.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_user_name_from_file_chain() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 0 || value > 100 { panic!("value must be between 0 and 100, got {}", value) }
        Guess { value }
    }

    pub fn get(&self) -> i32 { self.value }
}