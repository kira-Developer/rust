struct Person{
    name: String,
    age: u8
}
impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and I am {}." , self.name , self.age);
    }
}

fn main(){
let person = Person {name: String::from("kira") , age:18};
println!("{}", person.to_string());
}