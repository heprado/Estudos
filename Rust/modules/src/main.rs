mod person;

use crate::person::happy::HappyPerson;
use crate::person::sad::SadPerson;

use crate::person::Person;




fn main() {

    let person = Person::Happy(
        HappyPerson {
            name:String::from("A"),
        }
    );


    let person2 = Person::Sad(
        SadPerson {
            name:String::from("B"),
        }
    );

    person.say_hello();

    person2.say_hello();

}
