//! # 闭包(Closure)
//! Rust 学习闭包(closure)模块
use std::thread;
use std::time::Duration;

fn main() {
    let specified_value = 10;
    let random_number = 7;

    generate_workout(specified_value, random_number);

    let x = vec![1, 2, 3, 4];
    let catch = move |z| z == x;
    let y = vec![1, 2, 3, 4];
    println!("{}", catch(y));
    // println!("{:?}", x);
}

struct Cacher<T>
    where
        T: Fn(u32) -> u32, {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32, {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(value) => value,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

/// TO generate the workout with calling closure of simulate expensive calculation
///
/// ## Example
/// ```rust
///     generate_workout(10, 4);
/// ```
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("今天做{}个俯卧撑", expensive_result.value(intensity));
        println!("接下来做{}组仰卧起坐", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("今天休息")
        } else {
            println!("今天跑步{}分钟", expensive_result.value(intensity));
        }
    }
}

fn _simulate_expensive_calculation(intensity: i32) -> i32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
