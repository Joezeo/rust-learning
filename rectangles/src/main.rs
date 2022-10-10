#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // 关联函数（其实就相当于静态函数）
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 一个结构体可以有多个impl块
impl Rectangle {
    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.width > rec.width && self.height > rec.height
    }
}

fn main() {
    let rec1 = Rectangle {width:30, height:50};
    let rec2 = Rectangle {width:10, height:40};
    let rec3 = Rectangle {width:60, height:20};

    println!("The area of the rectangle is {}", rec1.area());
    println!("The rectangle is {:#?}", rec1);

    println!("Rec1 hold re2: {}", rec1.can_hold(&rec2));
    println!("Rec1 hold re3: {}", rec1.can_hold(&rec3));

    let square = Rectangle::square(40);
    println!("The area of the square is {}", square.area());
}


