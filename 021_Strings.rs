fn main(){
    let mut my_string = String::from("How's it going My name is Kira");

    // length
    println!("length: {}" , my_string.len());

    // Is Empty?

    println!("String is empty? {}" ,my_string.is_empty());
    
    for i in my_string.split_whitespace(){
        println!("{}" , i);
    }
     // contains
    println!("Does the string conrain 'kira'? {}" , my_string.contains("Kira"));

    // push 
    my_string.push_str(" just a push string");
    println!("{}" , my_string);
    
}