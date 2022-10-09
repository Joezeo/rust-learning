use std::ops::Add;

fn main() {
    scalar_type();
    println!("{}", compound_type());
}

fn scalar_type() {
    let _guess: u8 = "42".parse().expect("Not a number!");

    let _x = 2.0; // default f64 单精度
    let _y:f32 = 3.14; // 双精度浮点

    let _ch = 'z';
}

fn compound_type() -> String{
    let tuple = ("abc", 1, 3.3);
    println!("tuple directly -> {}, {}, {}", tuple.0, tuple.1, tuple.2);

    let (x, y, z) = tuple;
    println!("tuple copy -> {}, {}, {}", x, y, z);

    let a:[u32;3] = [1, 2, 3];
    println!("Print array a");
    for i in a {
        println!("{}", i);
    }

    // 不加分号，不需要使用return
    String::new().add("Hello World!")
}
