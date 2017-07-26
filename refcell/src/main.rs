use std::cell::RefCell;

fn borrows_immutably(value: &i32) {
    println!("value is {:?}", value);
}

fn borrows_mutably(value: &mut i32) {
    *value += 17;
}

fn demo(cell: &RefCell<i32>) {
    borrows_immutably(&cell.borrow());
    borrows_mutably(&mut cell.borrow_mut());
    borrows_immutably(&cell.borrow());
}

fn main() {
    println!("Hello, world!");
    let value = RefCell::new(1);
    demo(&value);
    let v2 = Rc::new(RefCell::new(5));
    let a = Cons(v2.clone(), Rc::new(Nil));
    let shared_list = Rc::new(a);
    let b = Cons(Rc::new(RefCell::new(6)), shared_list.clone());
    let c = Cons(Rc::new(RefCell::new(10)), shared_list.clone());

    *v2.borrow_mut() += 10;
    println!("shared_list after = {:?}", shared_list);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    main2();
}

// ------------------------
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main2() {
    let leaf = Rc::new(Node {
                           value: 3,
                           parent: RefCell::new(Weak::new()),
                           children: RefCell::new(vec![]),
                       });
    let branch = Rc::new(Node {
                             value: 5,
                             parent: RefCell::new(Weak::new()),
                             children: RefCell::new(vec![leaf.clone()]),
                         });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
