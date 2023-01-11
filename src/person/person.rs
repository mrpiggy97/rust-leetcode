pub struct Person{
    pub name : String
}

impl Person{
    pub fn say_hello(&self){
        println!("hello my name is {}", self.name);
    }
}