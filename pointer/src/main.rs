use std::rc::Rc;
use crate::List::{Cons, Nil};
use crate::mybox::*;

mod mybox;
mod refcell;
mod rc_refcell;
mod node;

fn main() {
    // Box
    let bp = Box::new(3);
    println!("{}", bp);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let name = MyBox::new("Joke");
    hello(&name);

    let _c = CustomSmartPointer {data: "My stuff".to_string()};
    let _d = CustomSmartPointer {data: "Other stuff".to_string()};
    println!("CustomSmartPointers created.");

    // Rc (Reference Counter)
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("Strong count is {}", Rc::strong_count(&a));

    let _b = Cons(0, Rc::clone(&a));
    println!("Strong count is {}", Rc::strong_count(&a));
    let _c = Cons(-1, Rc::clone(&a));
    println!("Strong count is {}", Rc::strong_count(&a));
    {
        let _d = Rc::new(Cons(1, Rc::clone(&a)));
        println!("Strong count is {}", Rc::strong_count(&a));
    }
    println!("Strong count is {}", Rc::strong_count(&a));
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

