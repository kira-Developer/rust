use std::fs::File;
use std::io::prelude::*;
fn main(){
let mut file = File::open("info.txt").expect("Can't open the file!!");
let mut contents = String::new();

file.read_to_string(&mut contents).expect("Can't read the file!!");
println!("{}" ,contents);
}