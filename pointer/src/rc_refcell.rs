#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::rc_refcell::List::Cons;

    use super::*;

    #[test]
    fn test_rc_cell_list() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(List::Nil)));

        let b = Rc::new(Cons(Rc::new(RefCell::new(6)), Rc::clone(&a)));
        let c = Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::clone(&a)));

        *value.borrow_mut() += 10;

        println!("{:?}", a);
        println!("{:?}", b);
        println!("{:?}", c);
    }

    #[test]
    fn test_cell() {
        let c = Cell::new("Hello");
        let v1 = c.get();
        c.set("World!");
        let v2 = c.get();
        println!("{} {}", v1, v2);
        assert_eq!(v1, "Hello");
        assert_eq!(v2, "World!");
    }

    #[test]
    fn test_refcell_change() {
        let a = Rc::new(RefCell::new("Hello ".to_string()));
        let b = Rc::clone(&a);
        let c = Rc::clone(&b);
        c.borrow_mut().push_str("World!");
        println!("{:?}, {:?}, {:?}", a, b, c)
    }
}