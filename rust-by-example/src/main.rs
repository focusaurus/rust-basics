fn to_tuple<T>(one: T, two: T) -> [T;2] {
    [one, two]
}

fn main (){
    println!("{:?}", to_tuple(17, 18));
    println!("{:?}", to_tuple(17.0, 18.2));
    println!("{:?}", to_tuple(false, true));
    println!("{:?}", to_tuple("bingo", "bango"));
    println!("{:?}", to_tuple(-4i8, -10i8));
}
