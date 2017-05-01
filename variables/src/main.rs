fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const Y: u8 = 17;
    println!("The value of Y is {Y}", Y = Y);
    let decimal: u32 = 42;
    let hex: u8 = 0xaa;
    let long_dec: u64 = 9_000_442;
    let bin: i8 = 0b01010101;
    println!("bin: {}", bin);
    let quo = 42.7 / 17.998;
    println!("quo: {}", quo);
    let (name, age, height) = ("Sara", 47, 5.75);
    println!("{name} {age}, {height} ft",
             name = name,
             age = age,
             height = height);
    // println!("{name} {age}, {height} ft");
    let tup = ('B', 'C', 'D');
    println!("foo {}", tup.2);
}
