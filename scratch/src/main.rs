use std::thread;
const data: &str = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

fn main() {
    let char_vec = data.chars()
            .filter(|c| c.is_numeric())
            .collect::<Vec<char>>();
    for (i, chunk) in char_vec.chunks(data.len() / 8).enumerate() {
        thread::spawn(move || -> u32 {
                          println!("{:?}", chunk);
                          i as u32
                      });
    }
}
/*
fn main2() {
    let numeric = data.chars().filter(|c| c.is_numeric());
    for (i, chunk) in numeric
            .collect::<Vec<char>>()
            .chunks(data.len() / 8)
            .enumerate() {
        thread::spawn(move || -> u32 {
                          println!("{:?}", chunk);
                          0
                      });
    }
}
*/
