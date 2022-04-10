fn main(){
    let numbers = 1..101;

    let names = ["kira" , "abdullah" , "someones"];
    for i in numbers {

        println!("the number is {}" , i);
    }
    println!("\n");
    for (index , i) in names.iter().enumerate() {

        println!("{}- name is {}",index, i);
    }
}
