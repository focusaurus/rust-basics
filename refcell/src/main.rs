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
}
