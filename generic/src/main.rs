fn largest(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn _x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Temp<T, U> {
    x: T,
    y: U,
}

impl<T, U> Temp<T, U> {
    fn mixup<W, H>(self, other: Temp<W, H>) -> Temp<T, H> {
        Temp {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let iarr = [1, 32, 11, 3, 123, 3, 33, 44, 123231];
    let carr = ['x', 'y', 'z', '0', '1', '2', '3', '4'];
    println!("{:?}", largest(&iarr));
    println!("{:?}", largest_generic(&carr));

    let _pi = Point {
        x: 3,
        y: 3,
    };

    let pf = Point {
        x: 4.0,
        y: 4.0,
    };

    println!("Distance from origin: {}", pf.distance_from_origin());

    let t1 = Temp { x: "Hello", y: 10 };
    let t2 = Temp { x: 10, y: "World!" };
    let mix = t1.mixup(t2);
    println!("{} {}", mix.x, mix.y);
}
