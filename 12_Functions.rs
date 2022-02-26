

fn main(){
    print_number_to(30);
    if is_even(32) {
        println!("It's even!");
    }
    }
    


fn print_number_to(number: u32){
    for i in 0..number{
        if is_even(i) {
            println!("{} is even" , i)
        }
        else {
            println!("{} is odd" , i)
        }
    }
}


fn is_even(number: u32) -> bool {
    return number % 2 == 0;
}