use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum ListCycle {
    Cons(i32, RefCell<Rc<ListCycle>>),
    Nil,
}

impl ListCycle {
    fn tail(&self) -> Option<&RefCell<Rc<ListCycle>>> {
        match self {
            ListCycle::Cons(_, item) => Some(item),
            ListCycle::Nil => None,
        }
    }
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after mut: {:?}", a);
    println!("b after mut: {:?}", b);
    println!("c after mut: {:?}", c);

    let a = Rc::new(ListCycle::Cons(5, RefCell::new(Rc::new(ListCycle::Nil))));

    println!("a initial ref count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(ListCycle::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a ref count after b creation = {}", Rc::strong_count(&a));
    println!("b initial ref count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b ref count after changing a = {}", Rc::strong_count(&b));
    println!("a ref count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}