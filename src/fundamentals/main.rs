// This is a main file and entry point.
mod fundamentals;
mod ferris;

use fundamentals::dog::Dog;
use fundamentals::practic::sum;

fn main() {
    let dog = Dog::new("Rocky", 2);
    dog.bark();
    print!("{} {} \n", dog.age, sum(4,6));
}