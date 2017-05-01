use std::env;

// fn canParse (input : String) -> bool {
//     match input.trim().parse() {
//             Ok(t) => t,
//             Err(e) => ,
//     }
// }
fn faren_to_celcius(faren: f32) -> f32 {
    (faren - 32.0) * (5.0 / 9.0)
}

fn main() {
    for temp in env::args().skip(1) {
        match temp.trim().parse::<f32>() {
            Ok(faren) => {
                println!("{:.1}", faren_to_celcius(faren));
            }
            Err(_) => {
                println!("IGNORING: {}", temp);
            }
        };
    }
}
