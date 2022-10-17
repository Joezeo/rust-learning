fn main() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }

    let v2: Vec<i32> = v1.iter().map(|val| val + 1).collect();
    println!("{:?}", v2);
}

#[test]
fn test_iterator() {
    let v1 = vec![1, 2, 3, 4];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), None);
    println!("{:?}", v1);
}

#[test]
fn test_iterator_sum() {
    let v1 = vec![1, 2, 3, 4];
    let v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum();
    assert_eq!(sum, 10);
}

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn _shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn test_filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: "sneaker".to_string() },
        Shoe { size: 13, style: "sandal".to_string() },
        Shoe { size: 10, style: "boot".to_string() },
    ];

    let filter_shoes = _shoes_in_my_size(shoes, 10);
    assert_eq!(vec![
        Shoe { size: 10, style: "sneaker".to_string() },
        Shoe { size: 10, style: "boot".to_string() },
    ], filter_shoes);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn _new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter::_new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn tet_counter_other_methods() {
    let sum: u32 = Counter::_new()
        .zip(Counter::_new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(sum, 18);
}
