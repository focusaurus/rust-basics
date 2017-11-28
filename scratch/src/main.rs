use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum Level {
    Error,
    Warning,
    Ok,
}

impl fmt::Display for Level {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out,
               "{}",
               match self {
                   &Level::Error => "🔥",
                   &Level::Warning => "⚠️",
                   &Level::Ok => "✓",
               })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum Check {
    Empty(Level, String),
    TooSmall(Level, String),
    TooBig(Level, String),
}

fn main() {
    let issues = [Check::Empty(Level::Error, "is empty".to_string()),
                  Check::TooBig(Level::Warning, "too big".to_string())];
    println!("{:?}",
             issues
                 .iter()
                 .any(|issue| match issue {
                          &Check::TooSmall(ref _l, ref _m) => true,
                          _ => false,
                      }));
    // match issue {
    //     Check::Empty(level, message) |
    //     Check::TooSmall(level, message) |
    //     Check::TooBig(level, message) => println!("{}: {}", level, message),
    // }
}
