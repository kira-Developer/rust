use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    
    for i in args.iter(){
        println!("{}" , i);
    }
}