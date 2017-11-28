extern crate chrono;
use chrono::{NaiveDate, Local};
// use chrono::NaiveDate::{signed_duration_since,from_ymd};
use std::fmt;

struct Person {
    name: String,
    born: NaiveDate,
}

impl Person {
    pub fn age(&self) -> i64 {
        NaiveDate::signed_duration_since(Local::today().naive_local(), self.born).num_days() / 365
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (age: {})", self.name, self.age())
    }
}

fn main() {
    let people = vec![Person {
                          name: "Alfred Matthew \"Weird Al\" Yankovic".to_string(),
                          born: NaiveDate::from_ymd(1959, 10, 23),
                      },
                      Person {
                          name: "Imogen Heap".to_string(),
                          born: NaiveDate::from_ymd(1977, 12, 9),
                      }];
    for person in people.iter() {
        println!("{}", person);
    }
}
