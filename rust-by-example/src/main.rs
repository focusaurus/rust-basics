fn main (){
    let one = Some(1);
    let nope: Option<u32> = None;
    let result = Some(3.14);
    let whoops: Result<u32, &str> = Err("W H O O P S");

    println!("{:?}", whoops.expect_err("They shouldn't have succeeded"));
}
