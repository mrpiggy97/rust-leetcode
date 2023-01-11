mod arithmetic;
mod arrays;
mod trees;
mod person;

use crate::person::person::Person;

fn main() {
    let result = 37/6;
    let remainder = 37 % 6;
    let num_str : String = format!("{}{}", result,remainder);
    println!("{}", num_str);
}
