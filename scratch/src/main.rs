extern crate chrono;
use chrono::{NaiveDate, Local};
use std::{cmp, fmt};

#[derive(Eq)]
struct Person {
    name: String,
    born: NaiveDate,
}

impl Person {
    pub fn new(name: String, year: i32, month: u32, day: u32) -> Self {
        Person {
            name,
            born: NaiveDate::from_ymd(year, month, day),
        }
    }

    pub fn age(&self) -> i64 {
        NaiveDate::signed_duration_since(Local::today().naive_local(), self.born).num_days() / 365
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Person) -> Option<cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Person) -> cmp::Ordering {
        self.born.cmp(&other.born)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.born == other.born
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (age: {})", self.name, self.age())
    }
}



fn main() {
    let mut people = vec![];
    people.push(Person::new("Imogen Heap".to_string(), 1977, 12, 9));
    people.push(Person::new("Fatboy Slim".to_string(), 1963, 7, 31));
    people.push(Person::new("Weird Al".to_string(), 1959, 10, 23));
    people.push(Person::new("Zoë Keating".to_string(), 1972, 2, 2));
    people.sort();
    for person in people.iter() {
        println!("{}", person);
    }
}
