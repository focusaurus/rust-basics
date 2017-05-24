#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn will_pass() {
        println!("{:?}", "will_pass running");
        assert_eq!(hey("foo"), 10);
    }
    #[test]
    fn will_fail() {
        println!("will_fail running");
        assert_eq!(hey("foo"), 9);
    }
}

fn hey(message: &str) -> u32 {
    println!("{:?}", message);
    10
}
