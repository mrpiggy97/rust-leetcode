mod arithmetic;
mod arrays;
mod person;
mod strings;

use crate::person::person::Person;

fn main() {
    let result = 37/6;
    let remainder = 37 % 6;
    let num_str : String = format!("{}{}", result,remainder);
    println!("{}", num_str);
    let fabian : Person = Person { name: String::from("fabian") };
    fabian.say_hello();
    let name : String = String::from("chris");
    let enum_name = name.into_bytes();
    let index : usize = 0;
    println!("{}", enum_name[index]);
}
