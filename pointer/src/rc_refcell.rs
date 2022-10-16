use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
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
}