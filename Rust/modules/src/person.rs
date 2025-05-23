use crate::person::happy::HappyPerson;
use crate::person::sad::SadPerson;

pub mod happy;

pub mod sad;


pub enum Person {
    Happy(HappyPerson),
    Sad(SadPerson),
}

impl Person {
    pub fn say_hello(&self) {
        match self {
            Person::Happy(person) => println!("Happy: {}", person.name),
            Person::Sad(person) => println!("Sad: {}",person.name),
        }
    }
}



