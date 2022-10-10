// #[derive(Debug)]
// enum IpAddrType {
//     IPV4(String),
//     IPV6(String)
// }
#[derive(Debug)]
enum IpAddrType {
    IPV4(u8, u8, u8, u8),
    IPV6(String),
}

// #[derive(Debug)]
// struct IPAddr {
//     ip: String,
//     kind: IpAddrType
// }

// 每个枚举都可以有不同的内嵌数据
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Resize { width: i32, height: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_coin(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State is {:#?}", state);
                25
            },
        }
    }
}

fn main() {
    let i4 = IpAddrType::IPV4(127, 0, 0, 1);
    let i6 = IpAddrType::IPV6("::1".to_string());

    println!("{:#?}", i4);
    println!("{:#?}", i6);

    let msg = Message::Write(String::from("Hello World"));
    msg.call();

    let mut some_number = Some(10);
    let _some_string = Some("Hello World");

    let _absent_number: Option<u32> = None;

    Coin::Quarter(UsState::Alabama).value_in_coin();
    some_number = plus_one(some_number);

    let u8_value = 0u8;
    match u8_value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => (),
    }

    let some_u8_value = Some(5u8);
    if let Some(3) = some_u8_value {
        println!("Three");
    } else if let Some(2) = some_u8_value {
        println!("Two");
    } else {
        println!("None");
    }
}

fn plus_one(x: Option<u32>) -> Option<u32> {
    match x {
        Some(x) => Some(x + 1),
        None => None,
    }
}
