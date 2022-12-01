use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, PartialOrd)]
struct Test {
    pub id: i32,
}
impl Ord for Test {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(Test { id: 2 });
    heap.push(Test { id: 4 });
    heap.push(Test { id: 3 });
    heap.push(Test { id: 1 });
    for i in heap.into_sorted_vec() {
        println!("{}", i.id);
    }
}
