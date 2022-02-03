// calling methods with the same name
#![allow(unused_variables)]
#![allow(dead_code)]
trait Pilot {
    fn fly();
}

trait Wizard {
    fn fly();
}

struct Human;

impl Human {
    fn fly() {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly() {
        println!("this is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly() {
        println!("Up!");
    }
}

pub fn main() {
    <Human as Wizard>::fly();
    Human::fly();
}
