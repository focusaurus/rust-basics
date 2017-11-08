#[derive(Debug)]
struct Ecks {
    pub quantity: usize,
}

fn call_by_imm_value(x: usize) {
    println!("{:?}", x);
}

fn call_by_mut_value(mut x: usize) {
    x += 1;
    println!("{:?}", x);
}

fn call_by_imm_ref(x: &Ecks) {
    println!("call_by_imm_ref {:?}", x);
}

fn call_by_mut_ref(x: &mut Ecks) {
    x.quantity += 1;
    println!("call_by_mut_ref {:?}", x);
}

fn main() {
    let x1 = 1;
    call_by_imm_value(x1);
    println!("x1 after {}", x1);
    let x2 = 20;
    call_by_mut_value(x2);
    println!("x2 after {}", x2);
    let x3 = Ecks { quantity: 30 };
    call_by_imm_ref(&x3);
    println!("x3 after {:?}", x3);
    let mut x4 = Ecks { quantity: 40 };
    call_by_mut_ref(&mut x4);
    println!("x3 after {:?}", x4);
}
